use num_traits::Zero;

use crate::{
    tx_mock::{
        BlockchainUpdate, TxCache, TxContext, TxContextStack, TxFunctionName, TxInput, TxLog,
        TxResult,
    },
    types::VMAddress,
    with_shared::Shareable,
};

use super::BlockchainVMRef;

impl BlockchainVMRef {
    /// Executes without builtin functions, directly on the contract or the given lambda closure.
    pub fn default_execution<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
        let mut tx_context_sh = Shareable::new(tx_context);

        if let Err(err) = tx_context_sh.tx_cache.transfer_egld_balance(
            &tx_context_sh.tx_input_box.from,
            &tx_context_sh.tx_input_box.to,
            &tx_context_sh.tx_input_box.egld_value,
        ) {
            return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
        }

        // skip for transactions coming directly from scenario json, which should all be coming from user wallets
        // TODO: reorg context logic
        let add_transfer_log = tx_context_sh.tx_input_box.from.is_smart_contract_address()
            && !tx_context_sh.tx_input_box.egld_value.is_zero();
        let transfer_value_log = if add_transfer_log {
            Some(TxLog {
                address: VMAddress::zero(), // TODO: figure out the real VM behavior
                endpoint: "transferValueOnly".into(),
                topics: vec![
                    tx_context_sh.tx_input_box.from.to_vec(),
                    tx_context_sh.tx_input_box.to.to_vec(),
                    tx_context_sh.tx_input_box.egld_value.to_bytes_be(),
                ],
                data: Vec::new(),
            })
        } else {
            None
        };

        // TODO: temporary, will convert to explicit builtin function first
        for esdt_transfer in tx_context_sh.tx_input_box.esdt_values.iter() {
            let transfer_result = tx_context_sh.tx_cache.transfer_esdt_balance(
                &tx_context_sh.tx_input_box.from,
                &tx_context_sh.tx_input_box.to,
                &esdt_transfer.token_identifier,
                esdt_transfer.nonce,
                &esdt_transfer.value,
            );
            if let Err(err) = transfer_result {
                return (TxResult::from_panic_obj(&err), BlockchainUpdate::empty());
            }
        }

        if tx_context_sh.tx_input_box.to.is_smart_contract_address()
            || !tx_context_sh.tx_input_box.func_name.is_empty()
        {
            TxContextStack::execute_on_vm_stack(&mut tx_context_sh, f);
        };

        let (mut tx_result, blockchain_updates) = tx_context_sh.into_inner().into_results();
        if let Some(tv_log) = transfer_value_log {
            tx_result.result_logs.insert(0, tv_log);
        }

        (tx_result, blockchain_updates)
    }

    pub fn deploy_contract(
        &self,
        mut tx_input: TxInput,
        contract_path: Vec<u8>,
        tx_cache: TxCache,
    ) -> (TxResult, VMAddress, BlockchainUpdate) {
        let new_address = tx_cache.get_new_address(&tx_input.from);
        tx_input.to = new_address.clone();
        tx_input.func_name = TxFunctionName::INIT;
        let tx_context = TxContext::new(self.clone(), tx_input, tx_cache);
        let tx_input_ref = &*tx_context.tx_input_box;

        if let Err(err) = tx_context
            .tx_cache
            .subtract_egld_balance(&tx_input_ref.from, &tx_input_ref.egld_value)
        {
            return (
                TxResult::from_panic_obj(&err),
                VMAddress::zero(),
                BlockchainUpdate::empty(),
            );
        }
        tx_context.create_new_contract(&new_address, contract_path, tx_input_ref.from.clone());
        tx_context
            .tx_cache
            .increase_egld_balance(&new_address, &tx_input_ref.egld_value);

        let tx_context = self.execute_tx_context(tx_context);
        let (tx_result, blockchain_updates) = tx_context.into_results();

        (tx_result, new_address, blockchain_updates)
    }
}
