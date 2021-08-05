#![allow(non_snake_case)]

use no_fuzz::no_fuzz;

#[test]
fn test() {
    no_fuzz(
        "base58_from_base58check",
        crypto_fuzzing::base58_from_base58check,
    );
    no_fuzz(
        "base58_to_base58check",
        crypto_fuzzing::base58_to_base58check,
    );
    no_fuzz("blake2b_digest_128", crypto_fuzzing::blake2b_digest_128);
    no_fuzz("blake2b_digest_160", crypto_fuzzing::blake2b_digest_160);
    no_fuzz("blake2b_digest_256", crypto_fuzzing::blake2b_digest_256);
    no_fuzz(
        "crypto_box_ProofOfWork_from_hex",
        crypto_fuzzing::crypto_box_ProofOfWork_from_hex,
    );
    no_fuzz(
        "crypto_box_PublicKey_from_hex",
        crypto_fuzzing::crypto_box_PublicKey_from_hex,
    );
    no_fuzz(
        "crypto_box_SecretKey_from_hex",
        crypto_fuzzing::crypto_box_SecretKey_from_hex,
    );
    no_fuzz("crypto_Nonce_new", crypto_fuzzing::crypto_Nonce_new);
    no_fuzz(
        "HashType_BlockHash_b58check_to_hash",
        crypto_fuzzing::HashType_BlockHash_b58check_to_hash,
    );
    no_fuzz(
        "HashType_BlockHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_BlockHash_bytes_and_string_convert,
    );
    no_fuzz(
        "HashType_BlockHash_hash_to_b58check",
        crypto_fuzzing::HashType_BlockHash_hash_to_b58check,
    );
    no_fuzz(
        "HashType_ChainId_bytes_and_string_convert",
        crypto_fuzzing::HashType_ChainId_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_BlockMetadataHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_BlockMetadataHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_ContextHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_ContextHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_ProtocolHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_ProtocolHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_OperationHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_OperationHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_OperationListListHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_OperationListListHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_OperationMetadataHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_OperationMetadataHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_OperationMetadataListListHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_OperationMetadataListListHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_CryptoboxPublicKeyHash_bytes_and_string_convert",
        crypto_fuzzing::HashType_CryptoboxPublicKeyHash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_ContractKt1Hash_bytes_and_string_convert",
        crypto_fuzzing::HashType_ContractKt1Hash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_ContractTz1Hash_bytes_and_string_convert",
        crypto_fuzzing::HashType_ContractTz1Hash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_ContractTz2Hash_bytes_and_string_convert",
        crypto_fuzzing::HashType_ContractTz2Hash_bytes_and_string_convert,
    );

    no_fuzz(
        "HashType_ContractTz3Hash_and_string_convert",
        crypto_fuzzing::HashType_ContractTz3Hash_and_string_convert,
    );

    no_fuzz(
        "HashType_PublicKeyEd25519_and_string_convert",
        crypto_fuzzing::HashType_PublicKeyEd25519_and_string_convert,
    );

    no_fuzz(
        "HashType_PublicKeySecp256k1_and_string_convert",
        crypto_fuzzing::HashType_PublicKeySecp256k1_and_string_convert,
    );

    no_fuzz(
        "HashType_PublicKeyP256_and_string_convert",
        crypto_fuzzing::HashType_PublicKeyP256_and_string_convert,
    );

    //no_fuzz("parse_query_string", rpc_fuzzing::parse_query_string);
    //no_fuzz("protocol_handler_test", rpc_fuzzing::protocol_handler_test);

    no_fuzz(
        "BlockHeaderWithHash_decode",
        storage_fuzzing::BlockHeaderWithHash_decode,
    );
    no_fuzz("Hash_decode", storage_fuzzing::Hash_decode);
    no_fuzz("MempoolKey_decode", storage_fuzzing::MempoolKey_decode);
    no_fuzz("Meta_decode", storage_fuzzing::Meta_decode);
    no_fuzz("MetaKey_decode", storage_fuzzing::MetaKey_decode);
    no_fuzz("OperationKey_decode", storage_fuzzing::OperationKey_decode);
    no_fuzz(
        "AckMessage_from_bytes",
        tezos_messages_fuzzing::AckMessage_from_bytes,
    );
    no_fuzz(
        "AdvertiseMessage_from_bytes",
        tezos_messages_fuzzing::AdvertiseMessage_from_bytes,
    );
    no_fuzz(
        "BinaryChunk_from_content",
        tezos_messages_fuzzing::BinaryChunk_from_content,
    );
    no_fuzz(
        "BlockHeader_from_bytes",
        tezos_messages_fuzzing::BlockHeader_from_bytes,
    );
    no_fuzz(
        "BlockHeaderMessage_from_bytes",
        tezos_messages_fuzzing::BlockHeaderMessage_from_bytes,
    );
    no_fuzz(
        "Component_from_bytes",
        tezos_messages_fuzzing::Component_from_bytes,
    );
    no_fuzz(
        "ConnectionMessage_from_bytes",
        tezos_messages_fuzzing::ConnectionMessage_from_bytes,
    );
    no_fuzz(
        "CurrentBranch_from_bytes",
        tezos_messages_fuzzing::CurrentBranch_from_bytes,
    );
    no_fuzz(
        "CurrentBranchMessage_from_bytes",
        tezos_messages_fuzzing::CurrentBranchMessage_from_bytes,
    );
    no_fuzz(
        "CurrentHeadMessage_from_bytes",
        tezos_messages_fuzzing::CurrentHeadMessage_from_bytes,
    );
    no_fuzz(
        "DeactivateMessage_from_bytes",
        tezos_messages_fuzzing::DeactivateMessage_from_bytes,
    );
    no_fuzz(
        "GetBlockHeadersMessage_from_bytes",
        tezos_messages_fuzzing::GetBlockHeadersMessage_from_bytes,
    );
    no_fuzz(
        "GetCurrentBranchMessage_from_bytes",
        tezos_messages_fuzzing::GetCurrentBranchMessage_from_bytes,
    );
    no_fuzz(
        "GetCurrentHeadMessage_from_bytes",
        tezos_messages_fuzzing::GetCurrentHeadMessage_from_bytes,
    );
    no_fuzz(
        "GetOperationsForBlocksMessage_from_bytes",
        tezos_messages_fuzzing::GetOperationsForBlocksMessage_from_bytes,
    );
    no_fuzz(
        "GetOperationsMessage_from_bytes",
        tezos_messages_fuzzing::GetOperationsMessage_from_bytes,
    );
    no_fuzz(
        "GetProtocolsMessage_from_bytes",
        tezos_messages_fuzzing::GetProtocolsMessage_from_bytes,
    );
    no_fuzz(
        "Mempool_from_bytes",
        tezos_messages_fuzzing::Mempool_from_bytes,
    );
    no_fuzz(
        "MetadataMessage_from_bytes",
        tezos_messages_fuzzing::MetadataMessage_from_bytes,
    );
    no_fuzz(
        "NetworkVersion_from_bytes",
        tezos_messages_fuzzing::NetworkVersion_from_bytes,
    );
    no_fuzz(
        "Operation_from_bytes",
        tezos_messages_fuzzing::Operation_from_bytes,
    );
    no_fuzz(
        "OperationMessage_from_bytes",
        tezos_messages_fuzzing::OperationMessage_from_bytes,
    );
    no_fuzz(
        "OperationsForBlock_from_bytes",
        tezos_messages_fuzzing::OperationsForBlock_from_bytes,
    );
    no_fuzz(
        "OperationsForBlocksMessage_from_bytes",
        tezos_messages_fuzzing::OperationsForBlocksMessage_from_bytes,
    );
    no_fuzz(
        "PeerMessageResponse_from_bytes",
        tezos_messages_fuzzing::PeerMessageResponse_from_bytes,
    );
    no_fuzz(
        "Protocol_from_bytes",
        tezos_messages_fuzzing::Protocol_from_bytes,
    );
    no_fuzz(
        "ProtocolMessage_from_bytes",
        tezos_messages_fuzzing::ProtocolMessage_from_bytes,
    );
    no_fuzz(
        "SwapMessage_from_bytes",
        tezos_messages_fuzzing::SwapMessage_from_bytes,
    );

    no_fuzz(
        "BinaryReader_read_Z",
        tezos_protocol_fuzzing::BinaryReader_read_Z,
    );
    no_fuzz(
        "BinaryReader_read_Mutez",
        tezos_protocol_fuzzing::BinaryReader_read_Mutez,
    );
    no_fuzz(
        "Protocol_get_constants_for_rpc",
        tezos_protocol_fuzzing::Protocol_get_constants_for_rpc,
    );

    // context fuzzing
    no_fuzz("context_api_fuzz", context_fuzzing::context_api_fuzz());
    no_fuzz(
        "workingtree_deserialize_serialize_fuzz",
        context_fuzzing::workingtree_deserialize_serialize_fuzz(),
    );
}
