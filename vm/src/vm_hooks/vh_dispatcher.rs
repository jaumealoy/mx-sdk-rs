use std::ffi::c_void;

use multiversx_vm_executor::{MemLength, MemPtr, VMHooks};

use super::vh_managed_types::{TxManagedTypesCell, VMHooksManagedTypes};

#[derive(Debug)]
pub struct VMHooksDispatcher {
    source: Box<dyn VMHooksManagedTypes>,
}

impl VMHooksDispatcher {
    pub fn new_managed_type_cell() -> Self {
        VMHooksDispatcher {
            source: Box::new(TxManagedTypesCell::default()),
        }
    }
}

#[allow(unused)]
impl VMHooks for VMHooksDispatcher {
    fn set_vm_hooks_ptr(&mut self, _vm_hooks_ptr: *mut c_void) {}

    fn get_gas_left(&self) -> i64 {
        panic!("Unavailable: get_gas_left")
    }

    fn get_sc_address(&self, result_offset: MemPtr) {
        panic!("Unavailable: get_sc_address");
    }

    fn get_owner_address(&self, result_offset: MemPtr) {
        panic!("Unavailable: get_owner_address");
    }

    fn get_shard_of_address(&self, address_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_shard_of_address")
    }

    fn is_smart_contract(&self, address_offset: MemPtr) -> i32 {
        panic!("Unavailable: is_smart_contract")
    }

    fn signal_error(&self, message_offset: MemPtr, message_length: MemLength) {
        panic!("Unavailable: signal_error");
    }

    fn get_external_balance(&self, address_offset: MemPtr, result_offset: MemPtr) {
        panic!("Unavailable: get_external_balance");
    }

    fn get_block_hash(&self, nonce: i64, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_block_hash")
    }

    fn get_esdt_balance(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
        result_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: get_esdt_balance")
    }

    fn get_esdt_nft_name_length(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
    ) -> i32 {
        panic!("Unavailable: get_esdt_nft_name_length")
    }

    fn get_esdt_nft_attribute_length(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
    ) -> i32 {
        panic!("Unavailable: get_esdt_nft_attribute_length")
    }

    fn get_esdt_nft_uri_length(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
    ) -> i32 {
        panic!("Unavailable: get_esdt_nft_uri_length")
    }

    fn get_esdt_token_data(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
        value_handle: i32,
        properties_offset: MemPtr,
        hash_offset: MemPtr,
        name_offset: MemPtr,
        attributes_offset: MemPtr,
        creator_offset: MemPtr,
        royalties_handle: i32,
        uris_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: get_esdt_token_data")
    }

    fn get_esdt_local_roles(&self, token_id_handle: i32) -> i64 {
        panic!("Unavailable: get_esdt_local_roles")
    }

    fn validate_token_identifier(&self, token_id_handle: i32) -> i32 {
        panic!("Unavailable: validate_token_identifier")
    }

    fn transfer_value(
        &self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        data_offset: MemPtr,
        length: MemLength,
    ) -> i32 {
        panic!("Unavailable: transfer_value")
    }

    fn transfer_value_execute(
        &self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: transfer_value_execute")
    }

    fn transfer_esdt_execute(
        &self,
        dest_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        value_offset: MemPtr,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: transfer_esdt_execute")
    }

    fn transfer_esdt_nft_execute(
        &self,
        dest_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        value_offset: MemPtr,
        nonce: i64,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: transfer_esdt_nft_execute")
    }

    fn multi_transfer_esdt_nft_execute(
        &self,
        dest_offset: MemPtr,
        num_token_transfers: i32,
        token_transfers_args_length_offset: MemPtr,
        token_transfer_data_offset: MemPtr,
        gas_limit: i64,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: multi_transfer_esdt_nft_execute")
    }

    fn create_async_call(
        &self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        data_offset: MemPtr,
        data_length: MemLength,
        success_offset: MemPtr,
        success_length: MemLength,
        error_offset: MemPtr,
        error_length: MemLength,
        gas: i64,
        extra_gas_for_callback: i64,
    ) -> i32 {
        panic!("Unavailable: create_async_call")
    }

    fn set_async_context_callback(
        &self,
        callback: MemPtr,
        callback_length: MemLength,
        data: MemPtr,
        data_length: MemLength,
        gas: i64,
    ) -> i32 {
        panic!("Unavailable: set_async_context_callback")
    }

    fn upgrade_contract(
        &self,
        dest_offset: MemPtr,
        gas_limit: i64,
        value_offset: MemPtr,
        code_offset: MemPtr,
        code_metadata_offset: MemPtr,
        length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) {
        panic!("Unavailable: upgrade_contract");
    }

    fn upgrade_from_source_contract(
        &self,
        dest_offset: MemPtr,
        gas_limit: i64,
        value_offset: MemPtr,
        source_contract_address_offset: MemPtr,
        code_metadata_offset: MemPtr,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) {
        panic!("Unavailable: upgrade_from_source_contract");
    }

    fn delete_contract(
        &self,
        dest_offset: MemPtr,
        gas_limit: i64,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) {
        panic!("Unavailable: delete_contract");
    }

    fn async_call(
        &self,
        dest_offset: MemPtr,
        value_offset: MemPtr,
        data_offset: MemPtr,
        length: MemLength,
    ) {
        panic!("Unavailable: async_call");
    }

    fn get_argument_length(&self, id: i32) -> i32 {
        panic!("Unavailable: get_argument_length")
    }

    fn get_argument(&self, id: i32, arg_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_argument")
    }

    fn get_function(&self, function_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_function")
    }

    fn get_num_arguments(&self) -> i32 {
        panic!("Unavailable: get_num_arguments")
    }

    fn storage_store(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> i32 {
        panic!("Unavailable: storage_store")
    }

    fn storage_load_length(&self, key_offset: MemPtr, key_length: MemLength) -> i32 {
        panic!("Unavailable: storage_load_length")
    }

    fn storage_load_from_address(
        &self,
        address_offset: MemPtr,
        key_offset: MemPtr,
        key_length: MemLength,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: storage_load_from_address")
    }

    fn storage_load(&self, key_offset: MemPtr, key_length: MemLength, data_offset: MemPtr) -> i32 {
        panic!("Unavailable: storage_load")
    }

    fn set_storage_lock(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        lock_timestamp: i64,
    ) -> i32 {
        panic!("Unavailable: set_storage_lock")
    }

    fn get_storage_lock(&self, key_offset: MemPtr, key_length: MemLength) -> i64 {
        panic!("Unavailable: get_storage_lock")
    }

    fn is_storage_locked(&self, key_offset: MemPtr, key_length: MemLength) -> i32 {
        panic!("Unavailable: is_storage_locked")
    }

    fn clear_storage_lock(&self, key_offset: MemPtr, key_length: MemLength) -> i32 {
        panic!("Unavailable: clear_storage_lock")
    }

    fn get_caller(&self, result_offset: MemPtr) {
        panic!("Unavailable: get_caller");
    }

    fn check_no_payment(&self) {
        panic!("Unavailable: check_no_payment");
    }

    fn get_call_value(&self, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_call_value")
    }

    fn get_esdt_value(&self, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_esdt_value")
    }

    fn get_esdt_value_by_index(&self, result_offset: MemPtr, index: i32) -> i32 {
        panic!("Unavailable: get_esdt_value_by_index")
    }

    fn get_esdt_token_name(&self, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_esdt_token_name")
    }

    fn get_esdt_token_name_by_index(&self, result_offset: MemPtr, index: i32) -> i32 {
        panic!("Unavailable: get_esdt_token_name_by_index")
    }

    fn get_esdt_token_nonce(&self) -> i64 {
        panic!("Unavailable: get_esdt_token_nonce")
    }

    fn get_esdt_token_nonce_by_index(&self, index: i32) -> i64 {
        panic!("Unavailable: get_esdt_token_nonce_by_index")
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
    ) -> i64 {
        panic!("Unavailable: get_current_esdt_nft_nonce")
    }

    fn get_esdt_token_type(&self) -> i32 {
        panic!("Unavailable: get_esdt_token_type")
    }

    fn get_esdt_token_type_by_index(&self, index: i32) -> i32 {
        panic!("Unavailable: get_esdt_token_type_by_index")
    }

    fn get_num_esdt_transfers(&self) -> i32 {
        panic!("Unavailable: get_num_esdt_transfers")
    }

    fn get_call_value_token_name(
        &self,
        call_value_offset: MemPtr,
        token_name_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: get_call_value_token_name")
    }

    fn get_call_value_token_name_by_index(
        &self,
        call_value_offset: MemPtr,
        token_name_offset: MemPtr,
        index: i32,
    ) -> i32 {
        panic!("Unavailable: get_call_value_token_name_by_index")
    }

    fn write_log(
        &self,
        data_pointer: MemPtr,
        data_length: MemLength,
        topic_ptr: MemPtr,
        num_topics: i32,
    ) {
        panic!("Unavailable: write_log");
    }

    fn write_event_log(
        &self,
        num_topics: i32,
        topic_lengths_offset: MemPtr,
        topic_offset: MemPtr,
        data_offset: MemPtr,
        data_length: MemLength,
    ) {
        panic!("Unavailable: write_event_log");
    }

    fn get_block_timestamp(&self) -> i64 {
        panic!("Unavailable: get_block_timestamp")
    }

    fn get_block_nonce(&self) -> i64 {
        panic!("Unavailable: get_block_nonce")
    }

    fn get_block_round(&self) -> i64 {
        panic!("Unavailable: get_block_round")
    }

    fn get_block_epoch(&self) -> i64 {
        panic!("Unavailable: get_block_epoch")
    }

    fn get_block_random_seed(&self, pointer: MemPtr) {
        panic!("Unavailable: get_block_random_seed");
    }

    fn get_state_root_hash(&self, pointer: MemPtr) {
        panic!("Unavailable: get_state_root_hash");
    }

    fn get_prev_block_timestamp(&self) -> i64 {
        panic!("Unavailable: get_prev_block_timestamp")
    }

    fn get_prev_block_nonce(&self) -> i64 {
        panic!("Unavailable: get_prev_block_nonce")
    }

    fn get_prev_block_round(&self) -> i64 {
        panic!("Unavailable: get_prev_block_round")
    }

    fn get_prev_block_epoch(&self) -> i64 {
        panic!("Unavailable: get_prev_block_epoch")
    }

    fn get_prev_block_random_seed(&self, pointer: MemPtr) {
        panic!("Unavailable: get_prev_block_random_seed");
    }

    fn finish(&self, pointer: MemPtr, length: MemLength) {
        panic!("Unavailable: finish");
    }

    fn execute_on_same_context(
        &self,
        gas_limit: i64,
        address_offset: MemPtr,
        value_offset: MemPtr,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: execute_on_same_context")
    }

    fn execute_on_dest_context(
        &self,
        gas_limit: i64,
        address_offset: MemPtr,
        value_offset: MemPtr,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: execute_on_dest_context")
    }

    fn execute_read_only(
        &self,
        gas_limit: i64,
        address_offset: MemPtr,
        function_offset: MemPtr,
        function_length: MemLength,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: execute_read_only")
    }

    fn create_contract(
        &self,
        gas_limit: i64,
        value_offset: MemPtr,
        code_offset: MemPtr,
        code_metadata_offset: MemPtr,
        length: MemLength,
        result_offset: MemPtr,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: create_contract")
    }

    fn deploy_from_source_contract(
        &self,
        gas_limit: i64,
        value_offset: MemPtr,
        source_contract_address_offset: MemPtr,
        code_metadata_offset: MemPtr,
        result_address_offset: MemPtr,
        num_arguments: i32,
        arguments_length_offset: MemPtr,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: deploy_from_source_contract")
    }

    fn get_num_return_data(&self) -> i32 {
        panic!("Unavailable: get_num_return_data")
    }

    fn get_return_data_size(&self, result_id: i32) -> i32 {
        panic!("Unavailable: get_return_data_size")
    }

    fn get_return_data(&self, result_id: i32, data_offset: MemPtr) -> i32 {
        panic!("Unavailable: get_return_data")
    }

    fn clean_return_data(&self) {
        panic!("Unavailable: clean_return_data");
    }

    fn delete_from_return_data(&self, result_id: i32) {
        panic!("Unavailable: delete_from_return_data");
    }

    fn get_original_tx_hash(&self, data_offset: MemPtr) {
        panic!("Unavailable: get_original_tx_hash");
    }

    fn get_current_tx_hash(&self, data_offset: MemPtr) {
        panic!("Unavailable: get_current_tx_hash");
    }

    fn get_prev_tx_hash(&self, data_offset: MemPtr) {
        panic!("Unavailable: get_prev_tx_hash");
    }

    fn managed_sc_address(&self, destination_handle: i32) {
        panic!("Unavailable: managed_sc_address");
    }

    fn managed_owner_address(&self, destination_handle: i32) {
        panic!("Unavailable: managed_owner_address");
    }

    fn managed_caller(&self, destination_handle: i32) {
        panic!("Unavailable: managed_caller");
    }

    fn managed_signal_error(&self, err_handle: i32) {
        panic!("Unavailable: managed_signal_error");
    }

    fn managed_write_log(&self, topics_handle: i32, data_handle: i32) {
        panic!("Unavailable: managed_write_log");
    }

    fn managed_get_original_tx_hash(&self, result_handle: i32) {
        panic!("Unavailable: managed_get_original_tx_hash");
    }

    fn managed_get_state_root_hash(&self, result_handle: i32) {
        panic!("Unavailable: managed_get_state_root_hash");
    }

    fn managed_get_block_random_seed(&self, result_handle: i32) {
        panic!("Unavailable: managed_get_block_random_seed");
    }

    fn managed_get_prev_block_random_seed(&self, result_handle: i32) {
        panic!("Unavailable: managed_get_prev_block_random_seed");
    }

    fn managed_get_return_data(&self, result_id: i32, result_handle: i32) {
        panic!("Unavailable: managed_get_return_data");
    }

    fn managed_get_multi_esdt_call_value(&self, multi_call_value_handle: i32) {
        panic!("Unavailable: managed_get_multi_esdt_call_value");
    }

    fn managed_get_esdt_balance(
        &self,
        address_handle: i32,
        token_id_handle: i32,
        nonce: i64,
        value_handle: i32,
    ) {
        panic!("Unavailable: managed_get_esdt_balance");
    }

    fn managed_get_esdt_token_data(
        &self,
        address_handle: i32,
        token_id_handle: i32,
        nonce: i64,
        value_handle: i32,
        properties_handle: i32,
        hash_handle: i32,
        name_handle: i32,
        attributes_handle: i32,
        creator_handle: i32,
        royalties_handle: i32,
        uris_handle: i32,
    ) {
        panic!("Unavailable: managed_get_esdt_token_data");
    }

    fn managed_async_call(
        &self,
        dest_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
    ) {
        panic!("Unavailable: managed_async_call");
    }

    fn managed_create_async_call(
        &self,
        dest_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        success_offset: MemPtr,
        success_length: MemLength,
        error_offset: MemPtr,
        error_length: MemLength,
        gas: i64,
        extra_gas_for_callback: i64,
        callback_closure_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_create_async_call")
    }

    fn managed_get_callback_closure(&self, callback_closure_handle: i32) {
        panic!("Unavailable: managed_get_callback_closure");
    }

    fn managed_upgrade_from_source_contract(
        &self,
        dest_handle: i32,
        gas: i64,
        value_handle: i32,
        address_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) {
        panic!("Unavailable: managed_upgrade_from_source_contract");
    }

    fn managed_upgrade_contract(
        &self,
        dest_handle: i32,
        gas: i64,
        value_handle: i32,
        code_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) {
        panic!("Unavailable: managed_upgrade_contract");
    }

    fn managed_delete_contract(&self, dest_handle: i32, gas_limit: i64, arguments_handle: i32) {
        panic!("Unavailable: managed_delete_contract");
    }

    fn managed_deploy_from_source_contract(
        &self,
        gas: i64,
        value_handle: i32,
        address_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        result_address_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_deploy_from_source_contract")
    }

    fn managed_create_contract(
        &self,
        gas: i64,
        value_handle: i32,
        code_handle: i32,
        code_metadata_handle: i32,
        arguments_handle: i32,
        result_address_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_create_contract")
    }

    fn managed_execute_read_only(
        &self,
        gas: i64,
        address_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_execute_read_only")
    }

    fn managed_execute_on_same_context(
        &self,
        gas: i64,
        address_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_execute_on_same_context")
    }

    fn managed_execute_on_dest_context(
        &self,
        gas: i64,
        address_handle: i32,
        value_handle: i32,
        function_handle: i32,
        arguments_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_execute_on_dest_context")
    }

    fn managed_multi_transfer_esdt_nft_execute(
        &self,
        dst_handle: i32,
        token_transfers_handle: i32,
        gas_limit: i64,
        function_handle: i32,
        arguments_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_multi_transfer_esdt_nft_execute")
    }

    fn managed_transfer_value_execute(
        &self,
        dst_handle: i32,
        value_handle: i32,
        gas_limit: i64,
        function_handle: i32,
        arguments_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_transfer_value_execute")
    }

    fn managed_is_esdt_frozen(&self, address_handle: i32, token_id_handle: i32, nonce: i64) -> i32 {
        panic!("Unavailable: managed_is_esdt_frozen")
    }

    fn managed_is_esdt_limited_transfer(&self, token_id_handle: i32) -> i32 {
        panic!("Unavailable: managed_is_esdt_limited_transfer")
    }

    fn managed_is_esdt_paused(&self, token_id_handle: i32) -> i32 {
        panic!("Unavailable: managed_is_esdt_paused")
    }

    fn managed_buffer_to_hex(&self, source_handle: i32, dest_handle: i32) {
        panic!("Unavailable: managed_buffer_to_hex");
    }

    fn big_float_new_from_parts(
        &self,
        integral_part: i32,
        fractional_part: i32,
        exponent: i32,
    ) -> i32 {
        panic!("Unavailable: big_float_new_from_parts")
    }

    fn big_float_new_from_frac(&self, numerator: i64, denominator: i64) -> i32 {
        panic!("Unavailable: big_float_new_from_frac")
    }

    fn big_float_new_from_sci(&self, significand: i64, exponent: i64) -> i32 {
        panic!("Unavailable: big_float_new_from_sci")
    }

    fn big_float_add(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        panic!("Unavailable: big_float_add");
    }

    fn big_float_sub(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        panic!("Unavailable: big_float_sub");
    }

    fn big_float_mul(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        panic!("Unavailable: big_float_mul");
    }

    fn big_float_div(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        panic!("Unavailable: big_float_div");
    }

    fn big_float_neg(&self, destination_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_neg");
    }

    fn big_float_clone(&self, destination_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_clone");
    }

    fn big_float_cmp(&self, op1_handle: i32, op2_handle: i32) -> i32 {
        panic!("Unavailable: big_float_cmp")
    }

    fn big_float_abs(&self, destination_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_abs");
    }

    fn big_float_sign(&self, op_handle: i32) -> i32 {
        panic!("Unavailable: big_float_sign")
    }

    fn big_float_sqrt(&self, destination_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_sqrt");
    }

    fn big_float_pow(&self, destination_handle: i32, op_handle: i32, exponent: i32) {
        panic!("Unavailable: big_float_pow");
    }

    fn big_float_floor(&self, dest_big_int_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_floor");
    }

    fn big_float_ceil(&self, dest_big_int_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_ceil");
    }

    fn big_float_truncate(&self, dest_big_int_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_float_truncate");
    }

    fn big_float_set_int64(&self, destination_handle: i32, value: i64) {
        panic!("Unavailable: big_float_set_int64");
    }

    fn big_float_is_int(&self, op_handle: i32) -> i32 {
        panic!("Unavailable: big_float_is_int")
    }

    fn big_float_set_big_int(&self, destination_handle: i32, big_int_handle: i32) {
        panic!("Unavailable: big_float_set_big_int");
    }

    fn big_float_get_const_pi(&self, destination_handle: i32) {
        panic!("Unavailable: big_float_get_const_pi");
    }

    fn big_float_get_const_e(&self, destination_handle: i32) {
        panic!("Unavailable: big_float_get_const_e");
    }

    fn big_int_get_unsigned_argument(&self, id: i32, destination_handle: i32) {
        panic!("Unavailable: big_int_get_unsigned_argument");
    }

    fn big_int_get_signed_argument(&self, id: i32, destination_handle: i32) {
        panic!("Unavailable: big_int_get_signed_argument");
    }

    fn big_int_storage_store_unsigned(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        source_handle: i32,
    ) -> i32 {
        panic!("Unavailable: big_int_storage_store_unsigned")
    }

    fn big_int_storage_load_unsigned(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        destination_handle: i32,
    ) -> i32 {
        panic!("Unavailable: big_int_storage_load_unsigned")
    }

    fn big_int_get_call_value(&self, destination_handle: i32) {
        panic!("Unavailable: big_int_get_call_value");
    }

    fn big_int_get_esdt_call_value(&self, destination: i32) {
        panic!("Unavailable: big_int_get_esdt_call_value");
    }

    fn big_int_get_esdt_call_value_by_index(&self, destination_handle: i32, index: i32) {
        panic!("Unavailable: big_int_get_esdt_call_value_by_index");
    }

    fn big_int_get_external_balance(&self, address_offset: MemPtr, result: i32) {
        panic!("Unavailable: big_int_get_external_balance");
    }

    fn big_int_get_esdt_external_balance(
        &self,
        address_offset: MemPtr,
        token_id_offset: MemPtr,
        token_id_len: MemLength,
        nonce: i64,
        result_handle: i32,
    ) {
        panic!("Unavailable: big_int_get_esdt_external_balance");
    }

    fn big_int_new(&self, small_value: i64) -> i32 {
        self.source.bi_new(small_value)
    }

    fn big_int_unsigned_byte_length(&self, reference_handle: i32) -> i32 {
        panic!("Unavailable: big_int_unsigned_byte_length")
    }

    fn big_int_signed_byte_length(&self, reference_handle: i32) -> i32 {
        panic!("Unavailable: big_int_signed_byte_length")
    }

    fn big_int_get_unsigned_bytes(&self, reference_handle: i32, byte_offset: MemPtr) -> i32 {
        panic!("Unavailable: big_int_get_unsigned_bytes")
    }

    fn big_int_get_signed_bytes(&self, reference_handle: i32, byte_offset: MemPtr) -> i32 {
        panic!("Unavailable: big_int_get_signed_bytes")
    }

    fn big_int_set_unsigned_bytes(
        &self,
        destination_handle: i32,
        byte_offset: MemPtr,
        byte_length: MemLength,
    ) {
        panic!("Unavailable: big_int_set_unsigned_bytes");
    }

    fn big_int_set_signed_bytes(
        &self,
        destination_handle: i32,
        byte_offset: MemPtr,
        byte_length: MemLength,
    ) {
        panic!("Unavailable: big_int_set_signed_bytes");
    }

    fn big_int_is_int64(&self, destination_handle: i32) -> i32 {
        self.source.bi_is_int64(destination_handle)
    }

    fn big_int_get_int64(&self, destination_handle: i32) -> i64 {
        self.source.bi_get_int64(destination_handle)
    }

    fn big_int_set_int64(&self, destination_handle: i32, value: i64) {
        self.source.bi_set_int64(destination_handle, value);
    }

    fn big_int_add(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_add(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_sub(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_sub(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_mul(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_mul(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_tdiv(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_t_div(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_tmod(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_t_mod(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_ediv(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        panic!("Not supported: big_int_ediv");
    }

    fn big_int_emod(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        panic!("Not supported: big_int_emod");
    }

    fn big_int_sqrt(&self, destination_handle: i32, op_handle: i32) {
        self.source.bi_sqrt(destination_handle, op_handle);
    }

    fn big_int_pow(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_pow(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_log2(&self, op_handle: i32) -> i32 {
        self.source.bi_log2(op_handle)
    }

    fn big_int_abs(&self, destination_handle: i32, op_handle: i32) {
        self.source.bi_abs(destination_handle, op_handle);
    }

    fn big_int_neg(&self, destination_handle: i32, op_handle: i32) {
        self.source.bi_neg(destination_handle, op_handle);
    }

    fn big_int_sign(&self, op_handle: i32) -> i32 {
        self.source.bi_sign(op_handle)
    }

    fn big_int_cmp(&self, op1_handle: i32, op2_handle: i32) -> i32 {
        self.source.bi_cmp(op1_handle, op2_handle)
    }

    fn big_int_not(&self, destination_handle: i32, op_handle: i32) {
        panic!("Unavailable: big_int_not");
    }

    fn big_int_and(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_and(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_or(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_or(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_xor(&self, destination_handle: i32, op1_handle: i32, op2_handle: i32) {
        self.source
            .bi_xor(destination_handle, op1_handle, op2_handle);
    }

    fn big_int_shr(&self, destination_handle: i32, op_handle: i32, bits: i32) {
        self.source
            .bi_shr(destination_handle, op_handle, bits as usize);
    }

    fn big_int_shl(&self, destination_handle: i32, op_handle: i32, bits: i32) {
        self.source
            .bi_shl(destination_handle, op_handle, bits as usize);
    }

    fn big_int_finish_unsigned(&self, reference_handle: i32) {
        panic!("Unavailable: big_int_finish_unsigned");
    }

    fn big_int_finish_signed(&self, reference_handle: i32) {
        panic!("Unavailable: big_int_finish_signed");
    }

    fn big_int_to_string(&self, big_int_handle: i32, destination_handle: i32) {
        panic!("Unavailable: big_int_to_string");
    }

    fn mbuffer_new(&self) -> i32 {
        panic!("Unavailable: mbuffer_new")
    }

    fn mbuffer_new_from_bytes(&self, data_offset: MemPtr, data_length: MemLength) -> i32 {
        panic!("Unavailable: mbuffer_new_from_bytes")
    }

    fn mbuffer_get_length(&self, m_buffer_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_get_length")
    }

    fn mbuffer_get_bytes(&self, m_buffer_handle: i32, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: mbuffer_get_bytes")
    }

    fn mbuffer_get_byte_slice(
        &self,
        source_handle: i32,
        starting_position: i32,
        slice_length: i32,
        result_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: mbuffer_get_byte_slice")
    }

    fn mbuffer_copy_byte_slice(
        &self,
        source_handle: i32,
        starting_position: i32,
        slice_length: i32,
        destination_handle: i32,
    ) -> i32 {
        panic!("Unavailable: mbuffer_copy_byte_slice")
    }

    fn mbuffer_eq(&self, m_buffer_handle1: i32, m_buffer_handle2: i32) -> i32 {
        panic!("Unavailable: mbuffer_eq")
    }

    fn mbuffer_set_bytes(
        &self,
        m_buffer_handle: i32,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> i32 {
        panic!("Unavailable: mbuffer_set_bytes")
    }

    fn mbuffer_set_byte_slice(
        &self,
        m_buffer_handle: i32,
        starting_position: i32,
        data_length: MemLength,
        data_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: mbuffer_set_byte_slice")
    }

    fn mbuffer_append(&self, accumulator_handle: i32, data_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_append")
    }

    fn mbuffer_append_bytes(
        &self,
        accumulator_handle: i32,
        data_offset: MemPtr,
        data_length: MemLength,
    ) -> i32 {
        panic!("Unavailable: mbuffer_append_bytes")
    }

    fn mbuffer_to_big_int_unsigned(&self, m_buffer_handle: i32, big_int_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_to_big_int_unsigned")
    }

    fn mbuffer_to_big_int_signed(&self, m_buffer_handle: i32, big_int_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_to_big_int_signed")
    }

    fn mbuffer_from_big_int_unsigned(&self, m_buffer_handle: i32, big_int_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_from_big_int_unsigned")
    }

    fn mbuffer_from_big_int_signed(&self, m_buffer_handle: i32, big_int_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_from_big_int_signed")
    }

    fn mbuffer_to_big_float(&self, m_buffer_handle: i32, big_float_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_to_big_float")
    }

    fn mbuffer_from_big_float(&self, m_buffer_handle: i32, big_float_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_from_big_float")
    }

    fn mbuffer_storage_store(&self, key_handle: i32, source_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_storage_store")
    }

    fn mbuffer_storage_load(&self, key_handle: i32, destination_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_storage_load")
    }

    fn mbuffer_storage_load_from_address(
        &self,
        address_handle: i32,
        key_handle: i32,
        destination_handle: i32,
    ) {
        panic!("Unavailable: mbuffer_storage_load_from_address");
    }

    fn mbuffer_get_argument(&self, id: i32, destination_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_get_argument")
    }

    fn mbuffer_finish(&self, source_handle: i32) -> i32 {
        panic!("Unavailable: mbuffer_finish")
    }

    fn mbuffer_set_random(&self, destination_handle: i32, length: i32) -> i32 {
        panic!("Unavailable: mbuffer_set_random")
    }

    fn managed_map_new(&self) -> i32 {
        panic!("Unavailable: managed_map_new")
    }

    fn managed_map_put(&self, m_map_handle: i32, key_handle: i32, value_handle: i32) -> i32 {
        panic!("Unavailable: managed_map_put")
    }

    fn managed_map_get(&self, m_map_handle: i32, key_handle: i32, out_value_handle: i32) -> i32 {
        panic!("Unavailable: managed_map_get")
    }

    fn managed_map_remove(&self, m_map_handle: i32, key_handle: i32, out_value_handle: i32) -> i32 {
        panic!("Unavailable: managed_map_remove")
    }

    fn managed_map_contains(&self, m_map_handle: i32, key_handle: i32) -> i32 {
        panic!("Unavailable: managed_map_contains")
    }

    fn small_int_get_unsigned_argument(&self, id: i32) -> i64 {
        panic!("Unavailable: small_int_get_unsigned_argument")
    }

    fn small_int_get_signed_argument(&self, id: i32) -> i64 {
        panic!("Unavailable: small_int_get_signed_argument")
    }

    fn small_int_finish_unsigned(&self, value: i64) {
        panic!("Unavailable: small_int_finish_unsigned");
    }

    fn small_int_finish_signed(&self, value: i64) {
        panic!("Unavailable: small_int_finish_signed");
    }

    fn small_int_storage_store_unsigned(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        value: i64,
    ) -> i32 {
        panic!("Unavailable: small_int_storage_store_unsigned")
    }

    fn small_int_storage_store_signed(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        value: i64,
    ) -> i32 {
        panic!("Unavailable: small_int_storage_store_signed")
    }

    fn small_int_storage_load_unsigned(&self, key_offset: MemPtr, key_length: MemLength) -> i64 {
        panic!("Unavailable: small_int_storage_load_unsigned")
    }

    fn small_int_storage_load_signed(&self, key_offset: MemPtr, key_length: MemLength) -> i64 {
        panic!("Unavailable: small_int_storage_load_signed")
    }

    fn int64get_argument(&self, id: i32) -> i64 {
        panic!("Unavailable: int64get_argument")
    }

    fn int64finish(&self, value: i64) {
        panic!("Unavailable: int64finish");
    }

    fn int64storage_store(&self, key_offset: MemPtr, key_length: MemLength, value: i64) -> i32 {
        panic!("Unavailable: int64storage_store")
    }

    fn int64storage_load(&self, key_offset: MemPtr, key_length: MemLength) -> i64 {
        panic!("Unavailable: int64storage_load")
    }

    fn sha256(&self, data_offset: MemPtr, length: MemLength, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: sha256")
    }

    fn managed_sha256(&self, input_handle: i32, output_handle: i32) -> i32 {
        panic!("Unavailable: managed_sha256")
    }

    fn keccak256(&self, data_offset: MemPtr, length: MemLength, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: keccak256")
    }

    fn managed_keccak256(&self, input_handle: i32, output_handle: i32) -> i32 {
        panic!("Unavailable: managed_keccak256")
    }

    fn ripemd160(&self, data_offset: MemPtr, length: MemLength, result_offset: MemPtr) -> i32 {
        panic!("Unavailable: ripemd160")
    }

    fn managed_ripemd160(&self, input_handle: i32, output_handle: i32) -> i32 {
        panic!("Unavailable: managed_ripemd160")
    }

    fn verify_bls(
        &self,
        key_offset: MemPtr,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: verify_bls")
    }

    fn managed_verify_bls(&self, key_handle: i32, message_handle: i32, sig_handle: i32) -> i32 {
        panic!("Unavailable: managed_verify_bls")
    }

    fn verify_ed25519(
        &self,
        key_offset: MemPtr,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: verify_ed25519")
    }

    fn managed_verify_ed25519(&self, key_handle: i32, message_handle: i32, sig_handle: i32) -> i32 {
        panic!("Unavailable: managed_verify_ed25519")
    }

    fn verify_custom_secp256k1(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
        hash_type: i32,
    ) -> i32 {
        panic!("Unavailable: verify_custom_secp256k1")
    }

    fn managed_verify_custom_secp256k1(
        &self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
        hash_type: i32,
    ) -> i32 {
        panic!("Unavailable: managed_verify_custom_secp256k1")
    }

    fn verify_secp256k1(
        &self,
        key_offset: MemPtr,
        key_length: MemLength,
        message_offset: MemPtr,
        message_length: MemLength,
        sig_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: verify_secp256k1")
    }

    fn managed_verify_secp256k1(
        &self,
        key_handle: i32,
        message_handle: i32,
        sig_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_verify_secp256k1")
    }

    fn encode_secp256k1_der_signature(
        &self,
        r_offset: MemPtr,
        r_length: MemLength,
        s_offset: MemPtr,
        s_length: MemLength,
        sig_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: encode_secp256k1_der_signature")
    }

    fn managed_encode_secp256k1_der_signature(
        &self,
        r_handle: i32,
        s_handle: i32,
        sig_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_encode_secp256k1_der_signature")
    }

    fn add_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        fst_point_xhandle: i32,
        fst_point_yhandle: i32,
        snd_point_xhandle: i32,
        snd_point_yhandle: i32,
    ) {
        panic!("Unavailable: add_ec");
    }

    fn double_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
    ) {
        panic!("Unavailable: double_ec");
    }

    fn is_on_curve_ec(&self, ec_handle: i32, point_xhandle: i32, point_yhandle: i32) -> i32 {
        panic!("Unavailable: is_on_curve_ec")
    }

    fn scalar_base_mult_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> i32 {
        panic!("Unavailable: scalar_base_mult_ec")
    }

    fn managed_scalar_base_mult_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_scalar_base_mult_ec")
    }

    fn scalar_mult_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> i32 {
        panic!("Unavailable: scalar_mult_ec")
    }

    fn managed_scalar_mult_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        point_xhandle: i32,
        point_yhandle: i32,
        data_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_scalar_mult_ec")
    }

    fn marshal_ec(
        &self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: marshal_ec")
    }

    fn managed_marshal_ec(
        &self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_marshal_ec")
    }

    fn marshal_compressed_ec(
        &self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: marshal_compressed_ec")
    }

    fn managed_marshal_compressed_ec(
        &self,
        x_pair_handle: i32,
        y_pair_handle: i32,
        ec_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_marshal_compressed_ec")
    }

    fn unmarshal_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> i32 {
        panic!("Unavailable: unmarshal_ec")
    }

    fn managed_unmarshal_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_unmarshal_ec")
    }

    fn unmarshal_compressed_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_offset: MemPtr,
        length: MemLength,
    ) -> i32 {
        panic!("Unavailable: unmarshal_compressed_ec")
    }

    fn managed_unmarshal_compressed_ec(
        &self,
        x_result_handle: i32,
        y_result_handle: i32,
        ec_handle: i32,
        data_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_unmarshal_compressed_ec")
    }

    fn generate_key_ec(
        &self,
        x_pub_key_handle: i32,
        y_pub_key_handle: i32,
        ec_handle: i32,
        result_offset: MemPtr,
    ) -> i32 {
        panic!("Unavailable: generate_key_ec")
    }

    fn managed_generate_key_ec(
        &self,
        x_pub_key_handle: i32,
        y_pub_key_handle: i32,
        ec_handle: i32,
        result_handle: i32,
    ) -> i32 {
        panic!("Unavailable: managed_generate_key_ec")
    }

    fn create_ec(&self, data_offset: MemPtr, data_length: MemLength) -> i32 {
        panic!("Unavailable: create_ec")
    }

    fn managed_create_ec(&self, data_handle: i32) -> i32 {
        panic!("Unavailable: managed_create_ec")
    }

    fn get_curve_length_ec(&self, ec_handle: i32) -> i32 {
        panic!("Unavailable: get_curve_length_ec")
    }

    fn get_priv_key_byte_length_ec(&self, ec_handle: i32) -> i32 {
        panic!("Unavailable: get_priv_key_byte_length_ec")
    }

    fn elliptic_curve_get_values(
        &self,
        ec_handle: i32,
        field_order_handle: i32,
        base_point_order_handle: i32,
        eq_constant_handle: i32,
        x_base_point_handle: i32,
        y_base_point_handle: i32,
    ) -> i32 {
        panic!("Unavailable: elliptic_curve_get_values")
    }
}
