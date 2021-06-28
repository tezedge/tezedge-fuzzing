#![allow(non_snake_case)]

no_fuzz::hfuzz!(tezos_messages_fuzzing::BlockHeaderMessage_from_bytes);

