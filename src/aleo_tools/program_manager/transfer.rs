use super::*;
use crate::{
    aleo_tools::program_manager::Credits,
    errors::{AvailError, AvailErrorType},
    models::{mobile_prover::ProverRequest, network::SupportedNetworks},
    service_clients::get_prover_client_with_session,
    utils::delegate_execution,
};
use security_framework::authorization;
use serde::{Deserialize, Serialize};
use snarkvm::ledger::{query::*, store::helpers::memory::BlockMemory};

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
                let (authorization, fee_authorization) = {
                    let api_client = self.api_client()?;
                    let rng = &mut rand::thread_rng();
                    let query: Query<N, BlockMemory<N>> = Query::from(api_client.base_url());
                    // Initialize a VM
                    let store = snarkvm::ledger::store::ConsensusStore::<
                        N,
                        snarkvm::ledger::store::helpers::memory::ConsensusMemory<N>,
                    >::open(None)?;
                    let vm = snarkvm::synthesizer::VM::from(store)?;
                    let transfer_type = TransferType::Public;
                    // check if the fee record is present
                    // if fee record is present, authorize the fee
                    // Create a new transaction.
                    let fee_authorization = {
                        if fee_record.is_some() {
                            let fee_record = fee_record.unwrap();
                            let fee_inputs = vec![
                                Value::Record(fee_record.clone()),
                                Value::from_str(&format!("{}u64", fee))?,
                                Value::from_str(&format!("{}u64", fee))?,
                                Value::from_str("9789517609field")?,
                            ];

                            let fee_authorization = vm.authorize(
                                &private_key,
                                "credits.aleo",
                                "fee_private",
                                fee_inputs.iter(),
                                rng,
                            )?;
                            Some(fee_authorization)
                        } else {
                            None
                        }
                    };
                    (
                        vm.authorize(
                            &private_key,
                            program_id,
                            transfer_function,
                            inputs.iter(),
                            rng,
                        )?,
                        fee_authorization,
                    )
                };
                println!("IN DELEGATE");
                let auth_bytes = ProverRequest::to_bytes_auth_object(authorization).unwrap();
                let fee_auth_bytes = match fee_authorization {
                    Some(fee_auth) => Some(ProverRequest::to_bytes_auth_object(fee_auth).unwrap()),
                    None => None,
                };
                let prover_request =
                    ProverRequest::new(sender, auth_bytes, network, fee_auth_bytes);
                let txn_string = delegate_execution(prover_request).await.unwrap();
                println!("txn_string: {:?}", txn_string);
                let txn = Transaction::from_str(&txn_string).unwrap();
                println!("txnid: {:?}", txn.id());
                txn
                // pass the auth object to prover service
            }
            false => {
                let rng = &mut rand::thread_rng();

                // Initialize a VM
                let store = ConsensusStore::<N, ConsensusMemory<N>>::open(None)?;
                let vm = VM::from(store)?;

                // Create a new transaction.
                vm.execute(
                    &private_key,
                    (program_id, transfer_function),
                    inputs.iter(),
                    fee_record,
                    fee,
                    Some(query),
                    rng,
                )?
            }
        };
        if delegate {
            return Ok(execution.id());
        }
        self.broadcast_transaction(execution.clone())?;

        Ok(execution.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleo_tools::{program_manager::Credits, test_utils::RECORD_2000000001_MICROCREDITS};
    use crate::models::constants::{TESTNET3_ADDRESS, TESTNET_ADDRESS, TESTNET_PRIVATE_KEY};
    use crate::models::network::SupportedNetworks;
    use crate::service_clients::{get_prover_client_with_session, SESSION};
    use crate::utils::delegate_execution;
    use security_framework::authorization;
    use snarkvm::ledger::{query::*, store::helpers::memory::BlockMemory};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_transfer() {
        // let st = SESSION.get_session_token().unwrap();
        SESSION.set_session_token("tylerDurden@0xf5".to_string());
        let private_key = PrivateKey::<Testnet3>::from_str(TESTNET_PRIVATE_KEY).unwrap();
        let api_client = AleoAPIClient::<Testnet3>::local_testnet3("3000", "116.203.142.0");
        let program_manager =
            ProgramManager::<Testnet3>::new(Some(private_key), None, Some(api_client), None)
                .unwrap();
        let amount = 1000000u64;
        let fee = 10000u64;
        let recipient_address = Address::from_str(TESTNET_ADDRESS).unwrap();
        let transfer_type = TransferType::PublicToPrivate;
        let password = Some("password");
        let amount_record = None;
        let fee_record = Some(Record::from_str(r"{owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,microcredits: 1000000u64.private,_nonce: 6359981118440619636307465025861597379883101966015424940295774216783421394007group.public}").unwrap());
        let program_id = "credits.aleo";
        let sender = "sender".to_string();
        let network = SupportedNetworks::Testnet3;
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
                delegate,
            )
            .await
            .unwrap();
        println!("RES ==> {:?}", result);
    }
}
