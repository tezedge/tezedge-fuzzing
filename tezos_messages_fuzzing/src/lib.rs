#![allow(non_snake_case)]

use ::no_fuzz::no_fuzz_all;

use tezos_messages::p2p::binary_message::BinaryChunk;
use tezos_messages::p2p::binary_message::{BinaryRead, BinaryWrite};
use tezos_messages::p2p::encoding::operation::GetOperationsMessage;
use tezos_messages::p2p::encoding::prelude::*;

pub fn BinaryChunk_from_content(data: &[u8]) {
    if let Ok(a) = BinaryChunk::from_content(&data) {
        let _ = a.raw();
        let _ = a.content();
    }
}

pub fn AckMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = AckMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn AdvertiseMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = AdvertiseMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn BlockHeader_from_bytes(data: &[u8]) {
    if let Ok(message) = BlockHeader::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn BlockHeaderMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = BlockHeaderMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn Component_from_bytes(data: &[u8]) {
    if let Ok(message) = Component::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn ConnectionMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = ConnectionMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn CurrentBranch_from_bytes(data: &[u8]) {
    if let Ok(message) = CurrentBranch::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn CurrentBranchMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = CurrentBranchMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn CurrentHeadMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = CurrentHeadMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn DeactivateMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = DeactivateMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn GetBlockHeadersMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = GetBlockHeadersMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn GetCurrentBranchMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = GetCurrentBranchMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn GetCurrentHeadMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = GetCurrentHeadMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn GetOperationsForBlocksMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = GetOperationsForBlocksMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn GetOperationsMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = GetOperationsMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn GetProtocolsMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = GetProtocolsMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn Mempool_from_bytes(data: &[u8]) {
    if let Ok(message) = Mempool::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn MetadataMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = MetadataMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn NetworkVersion_from_bytes(data: &[u8]) {
    if let Ok(message) = NetworkVersion::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn Operation_from_bytes(data: &[u8]) {
    if let Ok(message) = Operation::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn OperationMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = OperationMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn OperationsForBlock_from_bytes(data: &[u8]) {
    if let Ok(message) = OperationsForBlock::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn OperationsForBlocksMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = OperationsForBlocksMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn PeerMessageResponse_from_bytes(data: &[u8]) {
    if let Ok(message) = PeerMessageResponse::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn Protocol_from_bytes(data: &[u8]) {
    if let Ok(message) = Protocol::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn ProtocolMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = ProtocolMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

pub fn SwapMessage_from_bytes(data: &[u8]) {
    if let Ok(message) = SwapMessage::from_bytes(&data) {
        let encoded = message.as_bytes().expect("should be able to encode");
        assert_eq!(data, encoded);
    }
}

no_fuzz_all!{
    BinaryChunk_from_content,
    AckMessage_from_bytes,
    AdvertiseMessage_from_bytes,
    BlockHeader_from_bytes,
    BlockHeaderMessage_from_bytes,
    Component_from_bytes,
    ConnectionMessage_from_bytes,
    CurrentBranch_from_bytes,
    CurrentBranchMessage_from_bytes,
    CurrentHeadMessage_from_bytes,
    DeactivateMessage_from_bytes,
    GetBlockHeadersMessage_from_bytes,
    GetCurrentBranchMessage_from_bytes,
    GetCurrentHeadMessage_from_bytes,
    GetOperationsForBlocksMessage_from_bytes,
    GetOperationsMessage_from_bytes,
    GetProtocolsMessage_from_bytes,
    Mempool_from_bytes,
    MetadataMessage_from_bytes,
    NetworkVersion_from_bytes,
    Operation_from_bytes,
    OperationMessage_from_bytes,
    OperationsForBlock_from_bytes,
    OperationsForBlocksMessage_from_bytes,
    PeerMessageResponse_from_bytes,
    Protocol_from_bytes,
    ProtocolMessage_from_bytes,
    SwapMessage_from_bytes,
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        super::no_fuzz_all()
    }
}
