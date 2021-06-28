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

#[cfg(test)]
mod test {
    use no_fuzz::no_fuzz;

    #[test]
    fn test() {
        no_fuzz(
            "BlockHeaderWithHash_decode",
            crate::BlockHeaderWithHash_decode,
        );
        no_fuzz("Hash_decode", crate::Hash_decode);
        no_fuzz("MempoolKey_decode", crate::MempoolKey_decode);
        no_fuzz("Meta_decode", crate::Meta_decode);
        no_fuzz("MetaKey_decode", crate::MetaKey_decode);
        no_fuzz("OperationKey_decode", crate::OperationKey_decode);
    }
}
