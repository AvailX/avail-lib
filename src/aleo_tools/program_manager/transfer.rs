use std::{io::Bytes, vec};

use super::*;
use crate::{
    aleo_tools::program_manager::Credits,
    errors::{AvailError, AvailErrorType},
    models::{mobile_prover::ProverRequest, network::SupportedNetworks},
    service_clients::get_prover_client_with_session,
    utils::delegate_execution,
};
use rand::{
    rngs::{StdRng, ThreadRng},
    SeedableRng,
};
// use security_framework::authorization;
use serde::{Deserialize, Serialize};
use snarkvm::{
    circuit::environment::Public,
    ledger::{
        query::*,
        store::{helpers::memory::BlockMemory, BlockStorage, BlockStore, ConsensusStorage},
    },
    synthesizer::vm,
};

/// Transfer Type to Perform
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferType {
    /// Private credit transfer done via records associated with the account
    Private,
    /// Transfer credits from a record to the public account mapping for an Aleo account
    PrivateToPublic,
    /// Public credit transfer done via the account mapping
    Public,
    /// Transfer credits from the public account mapping to a record for an Aleo account
    PublicToPrivate,
}

impl<N: Network> ProgramManager<N> {
    //TODO - Remove unwraps
    /// Executes a transfer to the specified recipient_address with the specified amount and fee.
    /// Specify 0 for no fee.
    #[allow(clippy::too_many_arguments)]
    pub async fn transfer(
        &self,
        amount: u64,
        fee: u64,
        recipient_address: Address<N>,
        transfer_type: TransferType,
        password: Option<&str>,
        amount_record: Option<Record<N, Plaintext<N>>>,
        fee_record: Option<Record<N, Plaintext<N>>>,
        program_id: &str,
        sender: String,
        network: SupportedNetworks,
        delegate: bool,
    ) -> Result<N::TransactionID> {
        // Ensure records provided have enough credits to cover the transfer amount and fee
        if let Some(amount_record) = amount_record.as_ref() {
            ensure!(
                amount_record.microcredits()? >= amount,
                "Credits in amount record must greater than transfer amount specified"
            );
        }
        if let Some(fee_record) = fee_record.as_ref() {
            ensure!(
                fee_record.microcredits()? >= fee,
                "Credits in fee record must greater than fee specified"
            );
        }

        // Specify the network state query
        let query = Query::from(self.api_client.as_ref().unwrap().base_url());

        // Retrieve the private key.
        let private_key = self.get_private_key(password)?;

        // Prepare the inputs for a transfer.
        let (transfer_function, inputs) = match transfer_type {
            TransferType::Public => {
                let inputs = vec![
                    Value::from_str(&recipient_address.to_string())?,
                    Value::from_str(&format!("{}u64", amount))?,
                ];
                ("transfer_public", inputs)
            }
            TransferType::Private => {
                if amount_record.is_none() {
                    bail!("Amount record must be specified for private transfers");
                } else {
                    let inputs = vec![
                        Value::Record(amount_record.unwrap()),
                        Value::from_str(&recipient_address.to_string())?,
                        Value::from_str(&format!("{}u64", amount))?,
                    ];
                    ("transfer_private", inputs)
                }
            }
            TransferType::PublicToPrivate => {
                let inputs = vec![
                    Value::from_str(&recipient_address.to_string())?,
                    Value::from_str(&format!("{}u64", amount))?,
                ];
                ("transfer_public_to_private", inputs)
            }
            TransferType::PrivateToPublic => {
                if amount_record.is_none() {
                    bail!("Amount record must be specified for private transfers");
                } else {
                    let inputs = vec![
                        Value::Record(amount_record.unwrap()),
                        Value::from_str(&recipient_address.to_string())?,
                        Value::from_str(&format!("{}u64", amount))?,
                    ];
                    ("transfer_private_to_public", inputs)
                }
            }
        };
        // Generate the execution transaction
        let execution = match delegate {
            true => {
                let (authorization, fee_authorization, eId) = {
                    let api_client = self.api_client()?;
                    let mut rng = rand::thread_rng();
                    println!("||||| Base URL NUMBER 1 ===> {:?}", api_client.base_url());

                    // Initialize a VM
                    let store = snarkvm::ledger::store::ConsensusStore::<
                        N,
                        snarkvm::ledger::store::helpers::memory::ConsensusMemory<N>,
                    >::open(None)?;
                    let vm = VM::from(store)?;
                    let authorization = vm.authorize(
                        &private_key,
                        program_id,
                        transfer_function,
                        inputs.iter(),
                        &mut rng,
                    )?;
                    let execution_id = authorization.to_execution_id()?;
                    println!("====> EXID {:?}", execution_id.to_string());
                    let fee_authorization = {
                        if fee_record.is_some() {
                            let fee_record = fee_record.unwrap();
                            let fee_authorization = vm.authorize_fee_private(
                                &private_key,
                                fee_record,
                                fee,
                                fee,
                                execution_id,
                                &mut rng,
                            )?;
                            Some(fee_authorization)
                        } else {
                            let fee_authorization = vm.authorize_fee_public(
                                &private_key,
                                fee,
                                fee,
                                execution_id,
                                &mut rng,
                            )?;
                            Some(fee_authorization)
                        }
                    };
                    (authorization, fee_authorization, execution_id)
                };
                println!("DELEGATING EXECUTION TO AVAIL PROVER SERVICE");
                let auth_bytes =
                    ProverRequest::to_bytes_auth_object(authorization.clone()).unwrap();
                let fee_auth_bytes = match fee_authorization.clone() {
                    Some(fee_auth) => Some(ProverRequest::to_bytes_auth_object(fee_auth).unwrap()),
                    None => None,
                };
                let prover_request =
                    ProverRequest::new(sender.clone(), auth_bytes, network, fee_auth_bytes);
                let txn_string = delegate_execution(prover_request).await.unwrap();

                println!("txn_string: {:?}", txn_string);
                let txn = Transaction::from_str(&txn_string).unwrap();
                println!("txnid: {:?}", txn.id());
                txn
            }
            false => {
                let mut rng = rand::thread_rng();
                // Initialize a VM
                let store = ConsensusStore::<N, ConsensusMemory<N>>::open(None)?;
                let vm = VM::from(store)?;
                // Create a new transaction.
                println!("EXECUTING TRANSFER ON YOUR DEVICE");
                let exec = vm.execute(
                    &private_key,
                    (program_id, transfer_function),
                    inputs.iter(),
                    fee_record,
                    fee,
                    Some(query),
                    &mut rng,
                )?;
                exec
            }
        };
        // if delegate {
        //     return Ok(execution.id());
        // }

        self.broadcast_transaction(execution.clone())?;
        println!("EXECUTION ID: {:?}", execution);
        Ok(execution.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleo_tools::{program_manager::Credits, test_utils::RECORD_2000000001_MICROCREDITS};
    use crate::models::constants::{
        TESTNET3_ADDRESS, TESTNET3_ADDRESS_2, TESTNET3_PRIVATE_KEY, TESTNET_ADDRESS,
        TESTNET_PRIVATE_KEY,
    };
    use crate::models::network::SupportedNetworks;
    use crate::service_clients::{get_prover_client_with_session, SESSION};
    use crate::utils::delegate_execution;
    use chrono::Utc;
    use rusqlite::Transaction;
    use security_framework::authorization;
    use snarkvm::ledger::{query::*, store::helpers::memory::BlockMemory};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_transfer() {
        // let st = SESSION.get_session_token().unwrap();
        SESSION.set_session_token("tylerDurden@0xf5".to_string());
        let private_key = PrivateKey::<TestnetV0>::from_str(
            "APrivateKey1zkpEa57WrhvNVagKkja6mzU5waS4xFXidKtBNMweupft7JX",
        )
        .unwrap();
        // println!("P KEY: {:?}", private_key.get_address().to_string());
        let sender = "aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp".to_string();

        // let private_key = PrivateKey::<TestnetV0>::from_str(TESTNET3_PRIVATE_KEY).unwrap();
        let node_api_obscura = env!("TESTNET_API_OBSCURA");
        let base_url = format!(
            "https://aleo-testnetbeta.obscura.network/v1/{}",
            node_api_obscura
        );
        let api_client = AleoAPIClient::<TestnetV0>::new(&base_url, "testnet").unwrap();
        // let api_client = AleoAPIClient::<TestnetV0>::local_testnet3("3000", "116.203.142.0");
        let program_manager = ProgramManager::<TestnetV0>::new(
            Some(private_key),
            None,
            Some(api_client.clone()),
            None,
        )
        .unwrap();
        let amount = 10000u64;
        // let dddddd = 15188722u64;
        let fee = 963388u64;
        // let recipient_address = Address::from_str(TESTNET3_ADDRESS).unwrap();
        let recipient_address =
            Address::from_str("aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp")
                .unwrap();
        let transfer_type = TransferType::Public;
        let password = Some("tylerDurden@0xf5");
        let amount_record = None;
        const RECORD_MAINNET: &str = r"{owner:aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp.private,microcredits:3073224u64.private,_nonce:5199634801620992412289862193157265588660085109225328599563543525856471294537group.public}";
        let fee_record = None; //Some(Record::from_str(RECORD_MAINNET).unwrap()); ////Some(Record::from_str(r"{owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,microcredits: 1000000u64.private,_nonce: 6359981118440619636307465025861597379883101966015424940295774216783421394007group.public}").unwrap()); //None; //Some(Record::from_str(RECORD_MAINNET).unwrap()); //
        let program_id = "credits.aleo";
        let network = SupportedNetworks::Testnet;
        let delegate = true;
        let credits_mapping = match api_client
            .clone()
            .get_mapping_value(program_id, "account", &sender)
        {
            Ok(credits) => credits,
            Err(e) => {
                println!("ERROR: {:?}", e);
                return;
            }
        };
        println!("CREDITS MAPPING: {:?}", credits_mapping);
        // Modify the logic to handle time taken to execute the transfer
        let start = Instant::now();
        let result = program_manager
            .transfer(
                amount,
                fee,
                recipient_address,
                transfer_type,
                password,
                amount_record,
                fee_record,
                program_id,
                sender,
                network,
                !delegate,
            )
            .await
            .unwrap();
        let duration = start.elapsed();
        println!("Transaction successful in {:?}", duration);
        println!("RES ==> {:?}", result);
    }
    #[tokio::test]
    async fn test_transfer_with_broadcast() {
        // let st = SESSION.get_session_token().unwrap();
        SESSION.set_session_token("tylerDurden@0xf5".to_string());
        let private_key = PrivateKey::<TestnetV0>::from_str(
            "APrivateKey1zkpEa57WrhvNVagKkja6mzU5waS4xFXidKtBNMweupft7JX",
        )
        .unwrap();

        // let private_key = PrivateKey::<TestnetV0>::from_str(TESTNET3_PRIVATE_KEY).unwrap();
        let node_api_obscura = env!("TESTNET_API_OBSCURA");
        let base_url = format!(
            "https://aleo-testnetbeta.obscura.network/v1/{}",
            node_api_obscura
        );
        let api_client = AleoAPIClient::<TestnetV0>::new(&base_url, "testnet").unwrap();
        // let api_client = AleoAPIClient::<TestnetV0>::local_testnet3("3000", "116.203.142.0");
        let program_manager =
            ProgramManager::<TestnetV0>::new(Some(private_key), None, Some(api_client), None)
                .unwrap();
        let amount = 1000000u64;
        let fee = 10000u64;
        // let recipient_address = Address::from_str(TESTNET3_ADDRESS).unwrap();
        let recipient_address =
            Address::from_str("aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp")
                .unwrap();
        let transfer_type = TransferType::Public;
        let password = Some("tylerDurden@0xf5");
        let amount_record = None;
        const RECORD_MAINNET: &str = r"{owner:aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp.private,microcredits:5000000u64.private,_nonce:8225702631067250884087834370560624180419459511593007256346751473925039784459group.public}";
        let fee_record = None; //Some(Record::from_str(RECORD_MAINNET).unwrap()); ////Some(Record::from_str(r"{owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,microcredits: 1000000u64.private,_nonce: 6359981118440619636307465025861597379883101966015424940295774216783421394007group.public}").unwrap()); //None; //Some(Record::from_str(RECORD_MAINNET).unwrap()); //
        let program_id = "credits.aleo";
        let sender = "aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp".to_string();
        let network = SupportedNetworks::Testnet;
        let delegate = true;

        let result = program_manager
            .transfer(
                amount,
                fee,
                recipient_address,
                transfer_type,
                password,
                amount_record,
                fee_record,
                program_id,
                sender,
                network,
                false,
            )
            .await
            .unwrap();
        println!("RES ==> {:?}", result);
    }
    #[tokio::test]
    async fn test_trancaction() {
        let node_api_obscura = env!("TESTNET_API_OBSCURA");
        let base_url = format!(
            "https://aleo-testnetbeta.obscura.network/v1/{}",
            node_api_obscura
        );
        let api_client = AleoAPIClient::<TestnetV0>::new(&base_url, "testnet").unwrap();
        // let api_client = AleoAPIClient::<TestnetV0>::local_testnet3("3000", "116.203.142.0");

        let transaction_id =
            <snarkvm::prelude::TestnetV0 as snarkvm::prelude::Network>::TransactionID::from_str(
                "at18rs757ksn2n6njmlvlnk53slvjq62km8ck959th6hhdxj9qn4v8qxzvyr7",
            )
            .unwrap();
        let res = api_client.get_transaction(transaction_id).unwrap();
        println!("RES ==> {:?}", res);
    }

    // ------------------------ Functions that were once used to mock delegate execution ------------------------
    // THESE FN's THAT ARE COMMENTED BELOW MUST BE PLACED OUTSIDE TESTCASES TO BE USED
    // mocking server side fn as to reduce work from changing ProverRequest everytime
    // async fn mock_delegate_execution<N: Network>(
    //     auth: Authorization<N>,
    //     fee: Option<Authorization<N>>,
    //     rngs: &mut StdRng,
    //     vm: VM<N, ConsensusMemory<N>>,
    //     eid: Field<N>,
    //     sender: String,
    //     query: Query<N, BlockMemory<N>>,
    // ) -> Result<String> {
    //     // let _ = vm;
    //     println!("IN MOCK DELEGATE");
    //     let store = ConsensusStore::<N, ConsensusMemory<N>>::open(None)?;
    //     let vmn = VM::<N, ConsensusMemory<N>>::from(store)?;
    //     // // let api_client = setup_local_client::<N>(); //setup_client::<N>()?;
    //     let api_client = setup_client::<N>()?;
    //     println!("||||| Base URL Number 2 ===> {:?}", api_client.base_url());
    //     let mut rng = rand::thread_rng();

    //     let queryq: Query<N, BlockMemory<N>> = Query::from(api_client.base_url());
    //     println!("||||| AUTH IS ===> {:?}", auth.is_split());
    //     println!("||||| AUTH ===> {:?}", auth);
    //     println!("||||| FEE ===> {:?}", fee);
    //     let txn_return = vmn.execute_authorization(auth, fee, Some(queryq), &mut rng)?;
    //     println!(
    //         "|||| COntains ===> {:?}",
    //         vmn.contains_program(&ProgramID::<N>::from_str("credits.aleo")?)
    //     );
    //     let res = vm.finalize_store().get_value_speculative(
    //         ProgramID::from_str("credits.aleo")?,
    //         Identifier::from_str("account")?,
    //         &Plaintext::from(Literal::Address(Address::<N>::from_str(&sender).unwrap())),
    //     )?;
    //     println!("||||| RES MAP ===> {:?}", res);
    //     let check = match vmn.check_transaction(&txn_return, None, &mut rng) {
    //         Ok(x) => {
    //             println!("||||| check ===> {:?}", x);
    //         }
    //         Err(e) => {
    //             println!("||||| check ERROR ===> {:?}", e);
    //         }
    //     };

    //     let transitions = txn_return.transitions();
    //     broadcast_txn(txn_return.clone()).await?;

    //     Ok(txn_return.to_string())
    // }

    // async fn broadcast_txn<N: Network>(transaction: Transaction<N>) -> Result<String> {
    //     println!("IN BROADCAST");
    //     let transaction_type = if let Transaction::Deploy(..) = &transaction {
    //         "Deployment"
    //     } else {
    //         "Execute"
    //     };
    //     // let api_client = setup_local_client::<N>(); //setup_client::<N>()?;
    //     let api_client = setup_client::<N>()?;
    //     let result = api_client.transaction_broadcast(transaction);
    //     println!("||||| BROADCAST Result ===> {:?}", result);
    //     if result.is_ok() {
    //         println!(
    //             "✅ {} Transaction successfully posted to {}/{}",
    //             transaction_type,
    //             api_client.base_url(),
    //             api_client.network_id()
    //         );
    //         Ok(result?)
    //     } else {
    //         println!(
    //             "❌ {} Transaction failed to post to {}",
    //             transaction_type,
    //             api_client.base_url()
    //         );
    //         Ok(result?)
    //     }
    // }

    // pub fn setup_client<N: Network>() -> Result<AleoAPIClient<N>> {
    //     let node_api_obscura = env!("TESTNET_API_OBSCURA");

    //     println!("Node API Obscura: {:?}", node_api_obscura);

    //     let base_url = format!(
    //         "https://aleo-testnet3.obscura.build/v1/{}",
    //         node_api_obscura
    //     );

    //     let api_client = AleoAPIClient::<N>::new(&base_url, "testnet3")?;

    //     Ok(api_client)
    // }

    // THIS IS THE FN CALL TO THE MOCKED DELEGATE EXECUTION
    // let txn_string = mock_delegate_execution(
    //     authorization,
    //     fee_authorization,
    //     &mut rng,
    //     vm,
    //     eId,
    //     sender.clone(),
    //     query,
    // )
    // .await?;
    // let private_key = PrivateKey::<N>::from_str(
    //     "APrivateKey1zkpEa57WrhvNVagKkja6mzU5waS4xFXidKtBNMweupft7JX",
    // )?;
    // THIS LOGIC IS TO CHECK THE TXN
    // let res = vm.finalize_store().get_value_speculative(
    //     ProgramID::from_str("credits.aleo")?,
    //     Identifier::from_str("account")?,
    //     &Plaintext::from(Literal::Address(Address::<N>::from_str(&sender).unwrap())),
    // )?;
    // println!("||||| RES ===> {:?}", res);
    // let check = match vm.check_transaction(&exec, None, &mut rng) {
    //     Ok(x) => {
    //         println!("||||| check ===> {:?}", x);
    //     }
    //     Err(e) => {
    //         println!("||||| check ERROR ===> {:?}", e);
    //     }
    // };

    // recurring tests
    use futures::future::join_all;
    use std::time::{self, Instant};
    use tokio::time::Duration;
    #[tokio::test]
    async fn stress_test_ps() {
        SESSION.set_session_token("tylerDurden@0xf5".to_string());

        for _ in 1..100 {
            let res = get_prover_client_with_session(reqwest::Method::POST, "test")
                .unwrap()
                .send()
                .await
                .unwrap();
            println!("Prover Response{:?}", res);
            if res.status() == 200 {
                println!("Success {:?}", res);
            } else {
                println!("Error ");
            }
        }
    }
    #[tokio::test]
    async fn stress_test_transfer() {
        // let st = SESSION.get_session_token().unwrap();
        SESSION.set_session_token("tylerDurden@0xf5".to_string());
        let private_key = PrivateKey::<Testnet3>::from_str(
            "APrivateKey1zkpEa57WrhvNVagKkja6mzU5waS4xFXidKtBNMweupft7JX",
        )
        .unwrap();
        // println!("P KEY: {:?}", private_key.get_address().to_string());
        let sender = "aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp".to_string();

        // let private_key = PrivateKey::<Testnet3>::from_str(TESTNET3_PRIVATE_KEY).unwrap();
        let node_api_obscura = env!("TESTNET_API_OBSCURA");
        let base_url = format!(
            "https://aleo-testnet3.obscura.build/v1/{}",
            node_api_obscura
        );
        let api_client = AleoAPIClient::<Testnet3>::new(&base_url, "testnet3").unwrap();
        // let api_client = AleoAPIClient::<Testnet3>::local_testnet3("3000", "116.203.142.0");
        let program_manager = ProgramManager::<Testnet3>::new(
            Some(private_key),
            None,
            Some(api_client.clone()),
            None,
        )
        .unwrap();
        let amount = 10000u64;
        // let dddddd = 15188722u64;
        let fee = 963388u64;
        // let recipient_address = Address::from_str(TESTNET3_ADDRESS).unwrap();
        let recipient_address =
            Address::from_str("aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp")
                .unwrap();
        let transfer_type = TransferType::Public;
        let password = Some("tylerDurden@0xf5");
        let amount_record = None;
        const RECORD_MAINNET: &str = r"{owner:aleo18lmhpa6znqe4eqgnhqccze9awqtutlkh0aukd05k7pl52uu8cvysxqwurp.private,microcredits:3073224u64.private,_nonce:5199634801620992412289862193157265588660085109225328599563543525856471294537group.public}";
        let fee_record = None; //Some(Record::from_str(RECORD_MAINNET).unwrap()); ////Some(Record::from_str(r"{owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,microcredits: 1000000u64.private,_nonce: 6359981118440619636307465025861597379883101966015424940295774216783421394007group.public}").unwrap()); //None; //Some(Record::from_str(RECORD_MAINNET).unwrap()); //
        let program_id = "credits.aleo";
        let network = SupportedNetworks::Testnet3;
        let delegate = true;

        let credits_mapping = match api_client
            .clone()
            .get_mapping_value(program_id, "account", &sender)
        {
            Ok(credits) => credits,
            Err(e) => {
                println!("ERROR: {:?}", e);
                return;
            }
        };
        println!("CREDITS MAPPING: {:?}", credits_mapping);

        let mut handles = vec![];
        let mut total_time = Duration::new(0, 0);

        for _ in 0..10 {
            println!("_________________ITERATION_________________");
            let program_manager = program_manager.clone();
            let sender = sender.clone();
            let amount_record = amount_record.clone();
            let fee_record = fee_record.clone();
            let network = network.clone();
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                let result = program_manager
                    .transfer(
                        amount,
                        fee,
                        recipient_address,
                        transfer_type,
                        password,
                        amount_record,
                        fee_record,
                        program_id,
                        sender,
                        network,
                        delegate,
                    )
                    .await;
                println!(
                    "Time taken: {:?} || Timestamp: {:?}",
                    start.elapsed(),
                    Utc::now().timestamp()
                );
                let duration = start.elapsed();
                (result, duration)
            });

            handles.push(handle);
        }

        let results = join_all(handles).await;
        let mut successful_transfers = 0;
        let mut total_duration = Duration::new(0, 0);

        for result in results {
            match result {
                Ok((Ok(_), duration)) => {
                    println!("Transaction successful in {:?}", duration);
                    successful_transfers += 1;
                    total_duration += duration;
                }
                Ok((Err(e), _)) => {
                    println!("Transaction failed with error: {:?}", e);
                }
                Err(e) => {
                    println!("Task failed with error: {:?}", e);
                }
            }
        }

        if successful_transfers == 10 {
            println!("All transactions were successful");
        } else {
            println!("Some transactions failed");
        }

        let average_duration = total_duration / successful_transfers;
        println!("Total time for all transactions: {:?}", total_duration);
        println!("Average time per transaction: {:?}", average_duration);
    }
}
