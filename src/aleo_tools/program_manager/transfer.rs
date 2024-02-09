use super::*;
use crate::aleo_tools::program_manager::Credits;
use serde::{Deserialize, Serialize};
use snarkvm::ledger::query::*;

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
    pub fn transfer(
        &self,
        amount: u64,
        fee: u64,
        recipient_address: Address<N>,
        transfer_type: TransferType,
        password: Option<&str>,
        amount_record: Option<Record<N, Plaintext<N>>>,
        fee_record: Option<Record<N, Plaintext<N>>>,
        program_id: &str,
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

        // Generate the execution transaction
        let execution = {
            let rng = &mut rand::thread_rng();

            // Initialize a VM
            let store = ConsensusStore::<N, ConsensusMemory<N>>::open(None)?;
            let vm = VM::from(store)?;

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
        };

        self.broadcast_transaction(execution.clone())?;

        Ok(execution.id())
    }
}

#[cfg(test)]
mod tests {}
