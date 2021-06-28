#![no_main]
use libfuzzer_sys::fuzz_target;
use tezedge_fuzz::tezos_messages_fuzzing::fuzz_AckMessage_from_bytes;

fuzz_target!(|data: &[u8]| {
    fuzz_AckMessage_from_bytes(data);
});
