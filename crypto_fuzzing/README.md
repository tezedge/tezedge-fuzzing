# Crypto Functions Fuzz Targets

This project performs fuzz testing of cryptographic functions defined in the
`crypto` module of Tezedge
([link](https://github.com/tezedge/tezedge/blob/master/crypto/README.md)).

- `base58_from_base58check`
- `base58_to_base58check`
- `blake2b_digest_128`
- `blake2b_digest_160`
- `blake2b_digest_256`
- `crypto_box_ProofOfWork_from_hex`
- `crypto_box_PublicKey_from_hex`
- `crypto_box_SecretKey_from_hex`
- `crypto_Nonce_new`
- `HashType_BlockHash_b58check_to_hash`
- `HashType_BlockHash_bytes_and_string_convert`
- `HashType_BlockHash_hash_to_b58check`
- `HashType_BlockMetadataHash_bytes_and_string_convert`
- `HashType_ChainId_bytes_and_string_convert`
- `HashType_ContextHash_bytes_and_string_convert`
- `HashType_ContractKt1Hash_bytes_and_string_convert`
- `HashType_ContractTz1Hash_bytes_and_string_convert`
- `HashType_ContractTz2Hash_bytes_and_string_convert`
- `HashType_ContractTz3Hash_and_string_convert`
- `HashType_CryptoboxPublicKeyHash_bytes_and_string_convert`
- `HashType_OperationHash_bytes_and_string_convert`
- `HashType_OperationListListHash_bytes_and_string_convert`
- `HashType_OperationMetadataHash_bytes_and_string_convert`
- `HashType_OperationMetadataListListHash_bytes_and_string_convert`
- `HashType_ProtocolHash_bytes_and_string_convert`
- `HashType_PublicKeyEd25519_and_string_convert`
- `HashType_PublicKeyP256_and_string_convert`
- `HashType_PublicKeySecp256k1_and_string_convert`

## Running Individual Target

``` sh
cargo hfuzz run <target>
```

## Running All Targets in Sequence

``` sh
./run_all.sh
```
