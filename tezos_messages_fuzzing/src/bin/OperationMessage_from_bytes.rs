#![allow(non_snake_case)]

no_fuzz::hfuzz!(tezos_messages_fuzzing::OperationMessage_from_bytes);

