#[test]
fn test() {
    crypto_fuzzing::no_fuzz_all();
    storage_fuzzing::no_fuzz_all();
    tezos_messages_fuzzing::no_fuzz_all();
    tezos_protocol_fuzzing::no_fuzz_all();
    context_fuzzing::no_fuzz_all();
}
