#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::crypto_fuzzing::fuzz_HashType_BlockHash_bytes_and_string_convert;

fuzz_target!(|data: &[u8]| {
    fuzz_HashType_BlockHash_bytes_and_string_convert(data);
});
