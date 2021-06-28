#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::crypto_fuzzing::fuzz_HashType_BlockHash_b58check_to_hash;

fuzz_target!(|data: &[u8]| {
    fuzz_HashType_BlockHash_b58check_to_hash(data);
});
