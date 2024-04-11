use multiversx_sdk::data::transaction::TransactionInfo;

#[test]
pub fn deserialize_not_notarized_transaction() {
    let data = r#"
        {
            "data": {
                "transaction": {
                    "type": "normal",
                    "processingTypeOnSource": "BuiltInFunctionCall",
                    "processingTypeOnDestination": "SCInvoking",
                    "hash": "",
                    "nonce": 0,
                    "round": 0,
                    "epoch": 0,
                    "value": "0",
                    "receiver": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "sender": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "gasPrice": 1000000000,
                    "gasLimit": 20000000,
                    "gasUsed": 20000000,
                    "data": "==",
                    "signature": "",
                    "sourceShard": 0,
                    "destinationShard": 0,
                    "miniblockType": "",
                    "miniblockHash": "",
                    "timestamp": 1681142808,
                    "smartContractResults": [],
                    "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                        "events": []
                    },
                    "status": "success",
                    "tokens": [""],
                    "esdtValues": ["1"],
                    "receivers": ["erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3"],
                    "receiversShardIDs": [2],
                    "operation": "MultiESDTNFTTransfer",
                    "function": "listing",
                    "initiallyPaidFee": "669755000000000",
                    "fee": "669755000000000",
                    "chainID": "1",
                    "version": 1,
                    "options": 0
                }
            },
            "error": "",
            "code": "successful"
        }
    "#;

    let tx = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;

    assert!(tx.notarized_at_source_in_meta_nonce.is_none());
    assert!(tx.notarized_at_source_in_meta_hash.is_none());
    assert!(tx.notarized_at_destination_in_meta_nonce.is_none());
    assert!(tx.notarized_at_destination_in_meta_hash.is_none());
    assert!(tx.hyperblock_hash.is_none());
    assert!(tx.hyperblock_nonce.is_none());
    assert!(tx.block_hash.is_none());
    assert!(tx.block_nonce.is_none());
}

#[test]
pub fn deserialize_notarized_transaction() {
    let data = r#"
        {
            "data": {
                "transaction": {
                    "type": "normal",
                    "processingTypeOnSource": "BuiltInFunctionCall",
                    "processingTypeOnDestination": "SCInvoking",
                    "hash": "",
                    "nonce": 0,
                    "round": 0,
                    "epoch": 0,
                    "value": "0",
                    "receiver": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "sender": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "gasPrice": 1000000000,
                    "gasLimit": 20000000,
                    "gasUsed": 20000000,
                    "data": "==",
                    "signature": "",
                    "sourceShard": 0,
                    "destinationShard": 0,
                    "blockNonce": 14166447,
                    "blockHash": "",
                    "notarizedAtSourceInMetaNonce": 0,
                    "NotarizedAtSourceInMetaHash": "",
                    "notarizedAtDestinationInMetaNonce": 0,
                    "notarizedAtDestinationInMetaHash": "",
                    "miniblockType": "",
                    "miniblockHash": "",
                    "hyperblockNonce": 0,
                    "hyperblockHash": "",
                    "timestamp": 1681142808,
                    "smartContractResults": [],
                    "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                        "events": []
                    },
                    "status": "success",
                    "tokens": [""],
                    "esdtValues": ["1"],
                    "receivers": ["erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3"],
                    "receiversShardIDs": [2],
                    "operation": "MultiESDTNFTTransfer",
                    "function": "listing",
                    "initiallyPaidFee": "669755000000000",
                    "fee": "669755000000000",
                    "chainID": "1",
                    "version": 1,
                    "options": 0
                }
            },
            "error": "",
            "code": "successful"
        }
    "#;

    let tx = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;

    assert!(tx.notarized_at_source_in_meta_nonce.is_some());
    assert!(tx.notarized_at_source_in_meta_hash.is_some());
    assert!(tx.notarized_at_destination_in_meta_nonce.is_some());
    assert!(tx.notarized_at_destination_in_meta_hash.is_some());
    assert!(tx.hyperblock_hash.is_some());
    assert!(tx.hyperblock_nonce.is_some());
    assert!(tx.block_hash.is_some());
    assert!(tx.block_nonce.is_some());
}

#[test]
pub fn deserialize_a_transaction_with_token_transfers() {
    let data = r#"
        {
            "data": {
                "transaction": {
                    "type": "normal",
                    "processingTypeOnSource": "BuiltInFunctionCall",
                    "processingTypeOnDestination": "SCInvoking",
                    "hash": "",
                    "nonce": 0,
                    "round": 0,
                    "epoch": 0,
                    "value": "0",
                    "receiver": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "sender": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "gasPrice": 1000000000,
                    "gasLimit": 20000000,
                    "gasUsed": 20000000,
                    "data": "==",
                    "signature": "",
                    "sourceShard": 0,
                    "destinationShard": 0,
                    "blockNonce": 14166447,
                    "blockHash": "",
                    "notarizedAtSourceInMetaNonce": 0,
                    "NotarizedAtSourceInMetaHash": "",
                    "notarizedAtDestinationInMetaNonce": 0,
                    "notarizedAtDestinationInMetaHash": "",
                    "miniblockType": "",
                    "miniblockHash": "",
                    "hyperblockNonce": 0,
                    "hyperblockHash": "",
                    "timestamp": 1681142808,
                    "smartContractResults": [],
                    "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                        "events": []
                    },
                    "status": "success",
                    "receivers": ["erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3"],
                    "receiversShardIDs": [2],
                    "operation": "MultiESDTNFTTransfer",
                    "function": "listing",
                    "initiallyPaidFee": "669755000000000",
                    "fee": "669755000000000",
                    "chainID": "1",
                    "version": 1,
                    "options": 0,
                    "tokens": [
                        "USDC-c76f1f",
                        "AWESOME-12345"
                    ],
                    "esdtValues": [
                        "4643258",
                        "123456"
                    ]
                }
            },
            "error": "",
            "code": "successful"
        }
    "#;

    let tx = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;

    assert!(tx.tokens.is_some());

    let tokens = tx.tokens.unwrap();
    assert_eq!(tokens[0], "USDC-c76f1f");
    assert_eq!(tokens[1], "AWESOME-12345");

    assert!(tx.esdt_values.is_some());
    let esdt_values = tx.esdt_values.unwrap();
    assert_eq!(esdt_values[0], "4643258");
    assert_eq!(esdt_values[1], "123456");
}



#[test]
pub fn deserialize_a_transaction_without_token_transfers() {
    let data = r#"
        {
            "data": {
                "transaction": {
                    "type": "normal",
                    "processingTypeOnSource": "BuiltInFunctionCall",
                    "processingTypeOnDestination": "SCInvoking",
                    "hash": "",
                    "nonce": 0,
                    "round": 0,
                    "epoch": 0,
                    "value": "0",
                    "receiver": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "sender": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                    "gasPrice": 1000000000,
                    "gasLimit": 20000000,
                    "gasUsed": 20000000,
                    "data": "==",
                    "signature": "",
                    "sourceShard": 0,
                    "destinationShard": 0,
                    "blockNonce": 14166447,
                    "blockHash": "",
                    "notarizedAtSourceInMetaNonce": 0,
                    "NotarizedAtSourceInMetaHash": "",
                    "notarizedAtDestinationInMetaNonce": 0,
                    "notarizedAtDestinationInMetaHash": "",
                    "miniblockType": "",
                    "miniblockHash": "",
                    "hyperblockNonce": 0,
                    "hyperblockHash": "",
                    "timestamp": 1681142808,
                    "smartContractResults": [],
                    "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3",
                        "events": []
                    },
                    "status": "success",
                    "receivers": ["erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3"],
                    "receiversShardIDs": [2],
                    "operation": "MultiESDTNFTTransfer",
                    "function": "listing",
                    "initiallyPaidFee": "669755000000000",
                    "fee": "669755000000000",
                    "chainID": "1",
                    "version": 1,
                    "options": 0
                }
            },
            "error": "",
            "code": "successful"
        }
    "#;

    let tx = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;

    assert!(tx.tokens.is_none());
    assert!(tx.esdt_values.is_none());
}