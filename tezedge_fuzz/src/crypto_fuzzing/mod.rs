use crypto::base58::*;
use crypto::blake2b::*;
use crypto::crypto_box::*;
use crypto::hash::HashType;
use crypto::nonce::*;
use crypto::proof_of_work::ProofOfWork;
use hex::FromHex;

pub fn fuzz_base58_from_base58check(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(&data) {
        if let Ok(decoded) = s.from_base58check() {
            if let Ok(clear) = decoded.to_base58check() {
                assert_eq!(clear, s);
            }
        }
    }
}

pub fn fuzz_base58_to_base58check(data: &[u8]) {
    if let Ok(encoded) = data.to_base58check() {
        if let Ok(decoded) = encoded.from_base58check() {
            assert_eq!(decoded, data);
        }
    }
}

pub fn fuzz_blake2b_digest_128(data: &[u8]) {
    let _ = digest_128(&data);
}

pub fn fuzz_blake2b_digest_160(data: &[u8]) {
    let _ = digest_160(&data);
}
pub fn fuzz_blake2b_digest_256(data: &[u8]) {
    let _ = digest_256(&data);
}

pub fn fuzz_crypto_box_ProofOfWork_from_hex(data: &[u8]) {
    let _ = ProofOfWork::from_hex(&data);
}

pub fn fuzz_crypto_box_PublicKey_from_hex(data: &[u8]) {
    let _ = PublicKey::from_hex(&data);
}

pub fn fuzz_crypto_box_SecretKey_from_hex(data: &[u8]) {
    let _ = SecretKey::from_hex(&data);
}

pub fn fuzz_crypto_Nonce_new(data: &[u8]) {
    let nonce = Nonce::new(&data);
    nonce.increment();
    let _out = nonce.get_bytes();
}

pub fn fuzz_HashType_BlockHash_b58check_to_hash(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(&data) {
        //if let Ok(block) = HashType::BlockHash.b58check_to_hash(&s) {
        //let _ = chain_id_from_block_hash(&block);
        //}
        let _block = HashType::BlockHash.b58check_to_hash(&s);
    }
}

pub fn fuzz_HashType_BlockHash_bytes_and_string_convert(data: &[u8]) {
    if let Ok(s) = HashType::BlockHash.hash_to_b58check(&data) {
        if let Ok(byt) = HashType::BlockHash.b58check_to_hash(&s) {
            // TODO uncomment when assert remove
            // https://github.com/simplestaking/tezedge/blob/master/crypto/src/hash.rs#L135
            // https://github.com/simplestaking/tezedge/blob/master/crypto/src/hash.rs#L155
            assert_eq!(data, byt);
        }
    }
}

pub fn fuzz_HashType_BlockHash_hash_to_b58check(data: &[u8]) {
    let _ = HashType::BlockHash.hash_to_b58check(&data);
}
