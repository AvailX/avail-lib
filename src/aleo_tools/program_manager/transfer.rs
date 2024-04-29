use super::*;
use crate::{
    aleo_tools::program_manager::Credits,
    errors::{AvailError, AvailErrorType},
    models::{mobile_prover::ProverRequest, network::SupportedNetworks},
    service_clients::get_prover_client_with_session,
    utils::delegate_execution,
};
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
                let authorization = {
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

                    // Create a new transaction.
                    vm.authorize(
                        &private_key,
                        program_id,
                        transfer_function,
                        inputs.iter(),
                        rng,
                    )?
                };
                let auth_bytes = ProverRequest::to_bytes_auth_object(authorization).unwrap();
                let prover_request = ProverRequest::new(sender, auth_bytes, network, None);
                let txn_string = delegate_execution(prover_request).await.unwrap();
                println!("txn_string: {:?}", txn_string);
                let txn = Transaction::from_str(&txn_string).unwrap();
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
mod tests {}
