#![allow(non_snake_case)]

use crypto::base58::*;
use crypto::blake2b::*;
use crypto::crypto_box::*;
use crypto::hash::HashType;
use crypto::nonce::*;
use crypto::proof_of_work::ProofOfWork;
use hex::FromHex;

pub fn base58_from_base58check(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(&data) {
        if let Ok(decoded) = s.from_base58check() {
            if let Ok(clear) = decoded.to_base58check() {
                assert_eq!(clear, s);
            }
        }
    }
}

pub fn base58_to_base58check(data: &[u8]) {
    if let Ok(encoded) = data.to_base58check() {
        if let Ok(decoded) = encoded.from_base58check() {
            assert_eq!(decoded, data);
        }
    }
}

pub fn blake2b_digest_128(data: &[u8]) {
    let _ = digest_128(&data);
}

pub fn blake2b_digest_160(data: &[u8]) {
    let _ = digest_160(&data);
}

pub fn blake2b_digest_256(data: &[u8]) {
    let _ = digest_256(&data);
}

pub fn crypto_box_ProofOfWork_from_hex(data: &[u8]) {
    let _ = ProofOfWork::from_hex(&data);
}

pub fn crypto_box_PublicKey_from_hex(data: &[u8]) {
    let _ = PublicKey::from_hex(&data);
}

pub fn crypto_box_SecretKey_from_hex(data: &[u8]) {
    let _ = SecretKey::from_hex(&data);
}

pub fn crypto_Nonce_new(data: &[u8]) {
    let nonce = Nonce::new(&data);
    nonce.increment();
    let _out = nonce.get_bytes();
}

pub fn HashType_BlockHash_b58check_to_hash(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(&data) {
        //if let Ok(block) = HashType::BlockHash.b58check_to_hash(&s) {
        //let _ = chain_id_from_block_hash(&block);
        //}
        let _block = HashType::BlockHash.b58check_to_hash(&s);
    }
}

pub fn HashType_BlockHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::BlockHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::BlockHash.b58check_to_hash(&s) {
            // TODO uncomment when assert remove
            // https://github.com/simplestaking/tezedge/blob/master/crypto/src/hash.rs#L135
            // https://github.com/simplestaking/tezedge/blob/master/crypto/src/hash.rs#L155
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_BlockHash_hash_to_b58check(data: &[u8]) {
    let _ = HashType::BlockHash.hash_to_b58check(&data);
}

pub fn HashType_ChainId_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ChainId.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ChainId.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_BlockMetadataHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::BlockMetadataHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::BlockMetadataHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_ContextHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ContextHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ContextHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_ProtocolHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ProtocolHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ProtocolHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_OperationHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::OperationHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::OperationHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_OperationListListHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::OperationListListHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::OperationListListHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_OperationMetadataHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::OperationMetadataHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::OperationMetadataHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_OperationMetadataListListHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::OperationMetadataListListHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::OperationMetadataListListHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_CryptoboxPublicKeyHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::CryptoboxPublicKeyHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::CryptoboxPublicKeyHash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_ContractKt1Hash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ContractKt1Hash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ContractKt1Hash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_ContractTz1Hash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ContractTz1Hash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ContractTz1Hash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_ContractTz2Hash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ContractTz2Hash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ContractTz2Hash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_ContractTz3Hash_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::ContractTz3Hash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::ContractTz3Hash.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_PublicKeyEd25519_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::PublicKeyEd25519.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::PublicKeyEd25519.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_PublicKeySecp256k1_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::PublicKeySecp256k1.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::PublicKeySecp256k1.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

pub fn HashType_PublicKeyP256_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::PublicKeyP256.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::PublicKeyP256.b58check_to_hash(&s) {
            assert_eq!(data, byt);
        }
    }
}

no_fuzz::no_fuzz_all! {
    base58_from_base58check,
    base58_to_base58check,
    blake2b_digest_128,
    blake2b_digest_160,
    blake2b_digest_256,
    crypto_box_ProofOfWork_from_hex,
    crypto_box_PublicKey_from_hex,
    crypto_box_SecretKey_from_hex,
    crypto_Nonce_new,
    HashType_BlockHash_b58check_to_hash,
    HashType_BlockHash_bytes_and_string_convert,
    HashType_BlockHash_hash_to_b58check,
    HashType_ChainId_bytes_and_string_convert,
    HashType_BlockMetadataHash_bytes_and_string_convert,
    HashType_ContextHash_bytes_and_string_convert,
    HashType_ProtocolHash_bytes_and_string_convert,
    HashType_OperationHash_bytes_and_string_convert,
    HashType_OperationListListHash_bytes_and_string_convert,
    HashType_OperationMetadataHash_bytes_and_string_convert,
    HashType_OperationMetadataListListHash_bytes_and_string_convert,
    HashType_CryptoboxPublicKeyHash_bytes_and_string_convert,
    HashType_ContractKt1Hash_bytes_and_string_convert,
    HashType_ContractTz1Hash_bytes_and_string_convert,
    HashType_ContractTz2Hash_bytes_and_string_convert,
    HashType_ContractTz3Hash_and_string_convert,
    HashType_PublicKeyEd25519_and_string_convert,
    HashType_PublicKeySecp256k1_and_string_convert,
    HashType_PublicKeyP256_and_string_convert,
}

#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        super::no_fuzz_all();
    }
}
