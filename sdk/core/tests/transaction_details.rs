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
                    "smartContractResults": [
                        {
                            "hash": "648c0dd8fe6e6ddd9d68b41cc6ba3233bed143623c1c5a7839b75dad5427b8a9",
                            "nonce": 0,
                            "value": 0,
                            "receiver": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                            "sender": "erd1qqqqqqqqqqqqqpgqeel2kumf0r8ffyhth7pqdujjat9nx0862jpsg2pqaq",
                            "data": "ESDTTransfer@5745474c442d626434643739@011b37ba3417e581",
                            "prevTxHash": "61251d2ef8e89ead331963c7213150f6265c765905c66c6a0ae88c00bda159c3",
                            "originalTxHash": "61251d2ef8e89ead331963c7213150f6265c765905c66c6a0ae88c00bda159c3",
                            "gasLimit": 0,
                            "gasPrice": 1000000000,
                            "callType": 0,
                            "originalSender": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                            "logs": {
                              "address": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                              "events": [
                                {
                                  "address": "erd1qqqqqqqqqqqqqpgqeel2kumf0r8ffyhth7pqdujjat9nx0862jpsg2pqaq",
                                  "identifier": "ESDTTransfer",
                                  "topics": [
                                    "V0VHTEQtYmQ0ZDc5",
                                    "",
                                    "ARs3ujQX5YE=",
                                    "94KtwCoYrwniKsArYxPPiQw0mzJIj99YTmgcfffRq2o="
                                  ],
                                  "data": null,
                                  "additionalData": [
                                    "",
                                    "RVNEVFRyYW5zZmVy",
                                    "V0VHTEQtYmQ0ZDc5",
                                    "ARs3ujQX5YE="
                                  ]
                                },
                                {
                                  "address": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                                  "identifier": "writeLog",
                                  "topics": [
                                    "AAAAAAAAAAAFAM5+q3NpeM6Ukuu/ggbyUurLMzz6VIM="
                                  ],
                                  "data": "QDZmNmI=",
                                  "additionalData": [
                                    "QDZmNmI="
                                  ]
                                },
                                {
                                  "address": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                                  "identifier": "completedTxEvent",
                                  "topics": [
                                    "YSUdLvjonq0zGWPHITFQ9iZcdlkFxmxqCuiMAL2hWcM="
                                  ],
                                  "data": null,
                                  "additionalData": null
                                }
                              ]
                            },
                            "tokens": [
                              "WEGLD-bd4d79"
                            ],
                            "esdtValues": [
                              "79718691286541697"
                            ],
                            "operation": "ESDTTransfer"
                        }
                    ],
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
                    "smartContractResults": [
                        {
                            "hash": "648c0dd8fe6e6ddd9d68b41cc6ba3233bed143623c1c5a7839b75dad5427b8a9",
                            "nonce": 0,
                            "value": 0,
                            "receiver": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                            "sender": "erd1qqqqqqqqqqqqqpgqeel2kumf0r8ffyhth7pqdujjat9nx0862jpsg2pqaq",
                            "data": "ESDTTransfer@5745474c442d626434643739@011b37ba3417e581",
                            "prevTxHash": "61251d2ef8e89ead331963c7213150f6265c765905c66c6a0ae88c00bda159c3",
                            "originalTxHash": "61251d2ef8e89ead331963c7213150f6265c765905c66c6a0ae88c00bda159c3",
                            "gasLimit": 0,
                            "gasPrice": 1000000000,
                            "callType": 0,
                            "originalSender": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                            "logs": {
                              "address": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                              "events": [
                                {
                                  "address": "erd1qqqqqqqqqqqqqpgqeel2kumf0r8ffyhth7pqdujjat9nx0862jpsg2pqaq",
                                  "identifier": "ESDTTransfer",
                                  "topics": [
                                    "V0VHTEQtYmQ0ZDc5",
                                    "",
                                    "ARs3ujQX5YE=",
                                    "94KtwCoYrwniKsArYxPPiQw0mzJIj99YTmgcfffRq2o="
                                  ],
                                  "data": null,
                                  "additionalData": [
                                    "",
                                    "RVNEVFRyYW5zZmVy",
                                    "V0VHTEQtYmQ0ZDc5",
                                    "ARs3ujQX5YE="
                                  ]
                                },
                                {
                                  "address": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                                  "identifier": "writeLog",
                                  "topics": [
                                    "AAAAAAAAAAAFAM5+q3NpeM6Ukuu/ggbyUurLMzz6VIM="
                                  ],
                                  "data": "QDZmNmI=",
                                  "additionalData": [
                                    "QDZmNmI="
                                  ]
                                },
                                {
                                  "address": "erd177p2msp2rzhsnc32cq4kxy703yxrfxejfz8a7kzwdqw8ma734d4qtzk80t",
                                  "identifier": "completedTxEvent",
                                  "topics": [
                                    "YSUdLvjonq0zGWPHITFQ9iZcdlkFxmxqCuiMAL2hWcM="
                                  ],
                                  "data": null,
                                  "additionalData": null
                                }
                              ]
                            },
                            "tokens": [
                              "WEGLD-bd4d79"
                            ],
                            "esdtValues": [
                              "79718691286541697"
                            ],
                            "operation": "ESDTTransfer"
                        }
                    ],
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

    assert!(tx.smart_contract_results.as_ref().unwrap()[0].tokens.is_some());
    assert_eq!(tx.smart_contract_results.as_ref().unwrap()[0].tokens.as_ref().unwrap()[0], "WEGLD-bd4d79");
    assert_eq!(tx.smart_contract_results.as_ref().unwrap()[0].esdt_values.as_ref().unwrap()[0], "79718691286541697");

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