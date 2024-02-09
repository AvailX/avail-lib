// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use rand::rngs::ThreadRng;
use snarkvm::circuit::network::Aleo;
use snarkvm::ledger::{block::*, query::*, store::helpers::memory::BlockMemory};

impl<N: Network> ProgramManager<N> {
    /// Execute a program function on the Aleo Network.
    ///
    /// To run this function successfully, the program must already be deployed on the Aleo Network
    pub fn execute_program(
        &mut self,
        program_id: impl TryInto<ProgramID<N>>,
        function: impl TryInto<Identifier<N>>,
        inputs: impl ExactSizeIterator<Item = impl TryInto<Value<N>>>,
        priority_fee: u64,
        fee_record: Option<Record<N, Plaintext<N>>>,
        password: Option<&str>,
    ) -> Result<N::TransactionID> {
        // Ensure a network client is set, otherwise online execution is not possible
        ensure!(
            self.api_client.is_some(),
            "❌ Network client not set. A network client must be set before execution in order to send an execution transaction to the Aleo network"
        );

        // Check program and function have valid names
        let program_id = program_id
            .try_into()
            .map_err(|_| anyhow!("Invalid program ID"))?;
        let function_id = function
            .try_into()
            .map_err(|_| anyhow!("Invalid function name"))?;
        let function_name = function_id.to_string();

        // Get the program from chain, error if it doesn't exist
        let program = self
            .api_client()?
            .get_program(program_id)
            .map_err(|_| anyhow!("Program {program_id:?} does not exist on the Aleo Network. Try deploying the program first before executing."))?;

        // Create the execution transaction
        let private_key = self.get_private_key(password)?;
        let node_url = self.api_client.as_ref().unwrap().base_url().to_string();
        let transaction = Self::create_execute_transaction(
            &private_key,
            priority_fee,
            inputs,
            fee_record,
            &program,
            function_id,
            node_url,
            self.api_client()?,
        )?;

        // Broadcast the execution transaction to the network
        println!("Attempting to broadcast execution transaction for {program_id:?}");
        let execution = self.broadcast_transaction(transaction.clone());

        // Tell the user about the result of the execution before returning it
        if execution.is_ok() {
            println!("✅ Execution of function {function_name:?} from program {program_id:?}' broadcast successfully");
        } else {
            println!("❌ Execution of function {function_name:?} from program {program_id:?} failed to broadcast");
        }

        Ok(transaction.id())
    }

    /// Create an execute transaction without initializing a program manager instance
    #[allow(clippy::too_many_arguments)]
    pub fn create_execute_transaction(
        private_key: &PrivateKey<N>,
        priority_fee: u64,
        inputs: impl ExactSizeIterator<Item = impl TryInto<Value<N>>>,
        fee_record: Option<Record<N, Plaintext<N>>>,
        program: &Program<N>,
        function: impl TryInto<Identifier<N>>,
        node_url: String,
        api_client: &AleoAPIClient<N>,
    ) -> Result<Transaction<N>> {
        // Initialize an RNG and query object for the transaction
        let rng = &mut rand::thread_rng();
        let query = Query::from(node_url);

        // Check that the function exists in the program
        let function_name = function
            .try_into()
            .map_err(|_| anyhow!("Invalid function name"))?;
        let program_id = program.id();
        println!("Checking function {function_name:?} exists in {program_id:?}");
        ensure!(
            program.contains_function(&function_name),
            "Program {program_id:?} does not contain function {function_name:?}, aborting execution"
        );

        // Initialize the VM
        let vm = Self::initialize_vm(api_client, program, true)?;

        // Create an execution transaction
        vm.execute(
            private_key,
            (program_id, function_name),
            inputs,
            fee_record,
            priority_fee,
            Some(query),
            rng,
        )
    }

    /// Estimate the cost of executing a program with the given inputs in microcredits. The response
    /// will be in the form of (total_cost, (storage_cost, finalize_cost),execution)
    ///
    /// Disclaimer: Fee estimation is experimental and may not represent a correct estimate on any current or future network
    pub fn estimate_execution_fee<A: Aleo<Network = N>>(
        &self,
        program: &Program<N>,
        function: impl TryInto<Identifier<N>>,
        inputs: impl ExactSizeIterator<Item = impl TryInto<Value<N>>>,
    ) -> Result<(u64, (u64, u64), Execution<N>)> {
        let url = self.api_client.as_ref().map_or_else(
            || bail!("A network client must be configured to estimate a program execution fee"),
            |api_client| Ok(api_client.base_url()),
        )?;

        // Check that the function exists in the program
        let function_name = function
            .try_into()
            .map_err(|_| anyhow!("Invalid function name"))?;
        let program_id = program.id();
        println!("Checking function {function_name:?} exists in {program_id:?}");
        ensure!(
            program.contains_function(&function_name),
            "Program {program_id:?} does not contain function {function_name:?}, aborting execution"
        );

        // Create an ephemeral SnarkVM to store the programs
        // Initialize an RNG and query object for the transaction
        let rng = &mut rand::thread_rng();
        let query = Query::<N, BlockMemory<N>>::from(url);
        let vm = Self::initialize_vm(self.api_client()?, program, true)?;

        // Create an ephemeral private key for the sample execution
        let private_key = PrivateKey::<N>::new(rng)?;

        // Compute the authorization.
        let authorization = vm.authorize(&private_key, program_id, function_name, inputs, rng)?;

        let locator = Locator::new(*program_id, function_name);
        let (_, mut trace) = vm
            .process()
            .write()
            .execute::<A, ThreadRng>(authorization, rng)?;
        trace.prepare(query)?;
        let execution =
            trace.prove_execution::<A, _>(&locator.to_string(), &mut rand::thread_rng())?;
        // Add the fee data to the Avail Fee Estimation Microservice
        let (fee, (_storage_fee, _namespace_fee)) = execution_cost(&vm, &execution)?;
        Ok((fee, (_storage_fee, _namespace_fee), execution.clone()))
    }
    /// Estimate the finalize fee component for executing a function. This fee is additional to the
    /// size of the execution of the program in bytes. If the function does not have a finalize
    /// step, then the finalize fee is 0.
    ///
    /// Disclaimer: Fee estimation is experimental and may not represent a correct estimate on any current or future network
    pub fn estimate_finalize_fee(
        &self,
        program: &Program<N>,
        function: impl TryInto<Identifier<N>>,
    ) -> Result<u64> {
        let function_name = function
            .try_into()
            .map_err(|_| anyhow!("Invalid function name"))?;
        match program.get_function(&function_name)?.finalize_logic() {
            Some(finalize) => cost_in_microcredits(finalize),
            None => Ok(0u64),
        }
    }
}

// TODO: rewrite tests

#[cfg(test)]
mod tests {}
