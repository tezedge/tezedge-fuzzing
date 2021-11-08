#![allow(non_snake_case)]

use crypto::hash::Hash;
use storage::chain_meta_storage::MetaKey;
use storage::mempool_storage::MempoolKey;
use storage::operations_meta_storage::Meta;
use storage::operations_storage::OperationKey;
use storage::persistent::Decoder;
use storage::BlockHeaderWithHash;

pub fn BlockHeaderWithHash_decode(data: &[u8]) {
    let _ = BlockHeaderWithHash::decode(data);
}

pub fn Hash_decode(data: &[u8]) {
    let _ = Hash::decode(data);
}

pub fn MempoolKey_decode(data: &[u8]) {
    let _ = MempoolKey::decode(data);
}

pub fn Meta_decode(data: &[u8]) {
    let _ = Meta::decode(data);
}

pub fn MetaKey_decode(data: &[u8]) {
    let _ = MetaKey::decode(data);
}

pub fn OperationKey_decode(data: &[u8]) {
    let _ = OperationKey::decode(data);
}

no_fuzz::no_fuzz_all! {
    BlockHeaderWithHash_decode,
    Hash_decode,
    MempoolKey_decode,
    Meta_decode,
    MetaKey_decode,
    OperationKey_decode,
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        super::no_fuzz_all();
    }
}
