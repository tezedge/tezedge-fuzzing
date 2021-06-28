#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::crypto_fuzzing::fuzz_crypto_Nonce_new;

fuzz_target!(|data: &[u8]| {
    fuzz_crypto_Nonce_new(data);
});
