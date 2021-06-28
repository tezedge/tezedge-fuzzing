#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::crypto_fuzzing::fuzz_blake2b_digest_160;

fuzz_target!(|data: &[u8]| {
    fuzz_blake2b_digest_160(data);
});
