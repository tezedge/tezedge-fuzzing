#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::crypto_fuzzing::fuzz_base58_from_base58check;

fuzz_target!(|data: &[u8]| {
    fuzz_base58_from_base58check(data);
});
