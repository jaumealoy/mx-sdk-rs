// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    queue_repeat
    (
        init => init
        add => add
        count => count
        remove => remove
        bench => bench
        add_struct => add_struct
        count_struct => count_struct
        remove_struct => remove_struct
        bench_struct => bench_struct
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
