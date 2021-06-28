#![allow(non_snake_case)]

no_fuzz::hfuzz!(tezos_messages_fuzzing::ConnectionMessage_from_bytes);

