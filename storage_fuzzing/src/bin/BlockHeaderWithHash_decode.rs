#![allow(non_snake_case)]

no_fuzz::hfuzz!(storage_fuzzing::BlockHeaderWithHash_decode);
