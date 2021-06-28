#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::tezos_messages_fuzzing::fuzz_BinaryChunk_from_content;

fuzz_target!(|data: &[u8]| {
    fuzz_BinaryChunk_from_content(data);
});
