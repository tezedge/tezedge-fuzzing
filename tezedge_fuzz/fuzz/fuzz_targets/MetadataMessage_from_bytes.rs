#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::tezos_messages_fuzzing::fuzz_MetadataMessage_from_bytes;

fuzz_target!(|data: &[u8]| {
    fuzz_MetadataMessage_from_bytes(data);
});
