#![allow(non_snake_case)]

use tezos_messages::p2p::binary_message::BinaryChunk;
use tezos_messages::p2p::binary_message::{BinaryRead, BinaryWrite};
use tezos_messages::p2p::encoding::operation::GetOperationsMessage;
use tezos_messages::p2p::encoding::prelude::*;

use tezos_messages::protocol::{get_constants_for_rpc, SupportedProtocol};

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

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]
    use no_fuzz::no_fuzz;

    #[test]
    fn test() {
        no_fuzz("AckMessage_from_bytes", crate::AckMessage_from_bytes);
        no_fuzz(
            "AdvertiseMessage_from_bytes",
            crate::AdvertiseMessage_from_bytes,
        );
        no_fuzz("BinaryChunk_from_content", crate::BinaryChunk_from_content);
        no_fuzz("BlockHeader_from_bytes", crate::BlockHeader_from_bytes);
        no_fuzz(
            "BlockHeaderMessage_from_bytes",
            crate::BlockHeaderMessage_from_bytes,
        );
        no_fuzz("Component_from_bytes", crate::Component_from_bytes);
        no_fuzz(
            "ConnectionMessage_from_bytes",
            crate::ConnectionMessage_from_bytes,
        );
        no_fuzz("CurrentBranch_from_bytes", crate::CurrentBranch_from_bytes);
        no_fuzz(
            "CurrentBranchMessage_from_bytes",
            crate::CurrentBranchMessage_from_bytes,
        );
        no_fuzz(
            "CurrentHeadMessage_from_bytes",
            crate::CurrentHeadMessage_from_bytes,
        );
        no_fuzz(
            "DeactivateMessage_from_bytes",
            crate::DeactivateMessage_from_bytes,
        );
        no_fuzz(
            "GetBlockHeadersMessage_from_bytes",
            crate::GetBlockHeadersMessage_from_bytes,
        );
        no_fuzz(
            "GetCurrentBranchMessage_from_bytes",
            crate::GetCurrentBranchMessage_from_bytes,
        );
        no_fuzz(
            "GetCurrentHeadMessage_from_bytes",
            crate::GetCurrentHeadMessage_from_bytes,
        );
        no_fuzz(
            "GetOperationsForBlocksMessage_from_bytes",
            crate::GetOperationsForBlocksMessage_from_bytes,
        );
        no_fuzz(
            "GetOperationsMessage_from_bytes",
            crate::GetOperationsMessage_from_bytes,
        );
        no_fuzz(
            "GetProtocolsMessage_from_bytes",
            crate::GetProtocolsMessage_from_bytes,
        );
        no_fuzz("Mempool_from_bytes", crate::Mempool_from_bytes);
        no_fuzz(
            "MetadataMessage_from_bytes",
            crate::MetadataMessage_from_bytes,
        );
        no_fuzz(
            "NetworkVersion_from_bytes",
            crate::NetworkVersion_from_bytes,
        );
        no_fuzz("Operation_from_bytes", crate::Operation_from_bytes);
        no_fuzz(
            "OperationMessage_from_bytes",
            crate::OperationMessage_from_bytes,
        );
        no_fuzz(
            "OperationsForBlock_from_bytes",
            crate::OperationsForBlock_from_bytes,
        );
        no_fuzz(
            "OperationsForBlocksMessage_from_bytes",
            crate::OperationsForBlocksMessage_from_bytes,
        );
        no_fuzz(
            "PeerMessageResponse_from_bytes",
            crate::PeerMessageResponse_from_bytes,
        );
        no_fuzz("Protocol_from_bytes", crate::Protocol_from_bytes);
        no_fuzz(
            "ProtocolMessage_from_bytes",
            crate::ProtocolMessage_from_bytes,
        );
        no_fuzz("SwapMessage_from_bytes", crate::SwapMessage_from_bytes);
    }
}
