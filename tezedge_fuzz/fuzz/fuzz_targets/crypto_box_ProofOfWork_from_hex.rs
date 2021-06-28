#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::crypto_fuzzing::fuzz_crypto_box_ProofOfWork_from_hex;

fuzz_target!(|data: &[u8]| {
    fuzz_crypto_box_ProofOfWork_from_hex(data);
});
