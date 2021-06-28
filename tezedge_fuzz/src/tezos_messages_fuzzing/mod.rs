use tezos_messages::p2p::binary_message::BinaryChunk;
use tezos_messages::p2p::binary_message::BinaryMessage;
use tezos_messages::p2p::encoding::operation::GetOperationsMessage;
//use tezos_messages::p2p::encoding::operation_hashes_for_blocks::OperationHashesForBlocksMessage;
use tezos_messages::p2p::encoding::prelude::{
    AckMessage, AdvertiseMessage, BlockHeader, BlockHeaderMessage, Component, ConnectionMessage,
    CurrentBranch, CurrentBranchMessage, CurrentHeadMessage, DeactivateMessage,
    GetBlockHeadersMessage, GetCurrentBranchMessage, GetCurrentHeadMessage,
    GetOperationsForBlocksMessage, GetProtocolsMessage, Mempool, MetadataMessage, NetworkVersion,
    Operation, OperationMessage, OperationsForBlock, OperationsForBlocksMessage,
    PeerMessageResponse, Protocol, ProtocolMessage, SwapMessage,
};

pub fn fuzz_AckMessage_from_bytes(data: &[u8]) {
    let _ = AckMessage::from_bytes(&data);
}

pub fn fuzz_AdvertiseMessage_from_bytes(data: &[u8]) {
    let _ = AdvertiseMessage::from_bytes(&data);
}

pub fn fuzz_BinaryChunk_from_content(data: &[u8]) {
    if let Ok(a) = BinaryChunk::from_content(&data) {
        let _ = a.raw();
        let _ = a.content();
    }
}
pub fn fuzz_BlockHeader_from_bytes(data: &[u8]) {
    let _ = BlockHeader::from_bytes(&data);
}

pub fn fuzz_BlockHeaderMessage_from_bytes(data: &[u8]) {
    let _ = BlockHeaderMessage::from_bytes(&data);
}

pub fn fuzz_Component_from_bytes(data: &[u8]) {
    let _ = Component::from_bytes(&data);
}

pub fn fuzz_ConnectionMessage_from_bytes(data: &[u8]) {
    let _ = ConnectionMessage::from_bytes(&data);
}

pub fn fuzz_CurrentBranch_from_bytes(data: &[u8]) {
    let _ = CurrentBranch::from_bytes(&data);
}

pub fn fuzz_CurrentBranchMessage_from_bytes(data: &[u8]) {
    let _ = CurrentBranchMessage::from_bytes(&data);
}

pub fn fuzz_CurrentHeadMessage_from_bytes(data: &[u8]) {
    let _ = CurrentHeadMessage::from_bytes(&data);
}

pub fn fuzz_DeactivateMessage_from_bytes(data: &[u8]) {
    let _ = DeactivateMessage::from_bytes(&data);
}

pub fn fuzz_GetBlockHeadersMessage_from_bytes(data: &[u8]) {
    let _ = GetBlockHeadersMessage::from_bytes(&data);
}

pub fn fuzz_GetCurrentBranchMessage_from_bytes(data: &[u8]) {
    let _ = GetCurrentBranchMessage::from_bytes(&data);
}

pub fn fuzz_GetCurrentHeadMessage_from_bytes(data: &[u8]) {
    let _ = GetCurrentHeadMessage::from_bytes(&data);
}

//pub fn GetOperationHashesForBlocksMessage_from_bytes(data: &[u8]) {
//    let _ = GetOperationHashesForBlocksMessage::from_bytes(&data);
//}

pub fn fuzz_GetOperationsForBlocksMessage_from_bytes(data: &[u8]) {
    let _ = GetOperationsForBlocksMessage::from_bytes(&data);
}

pub fn fuzz_GetOperationsMessage_from_bytes(data: &[u8]) {
    let _ = GetOperationsMessage::from_bytes(&data);
}

pub fn fuzz_GetProtocolsMessage_from_bytes(data: &[u8]) {
    let _ = GetProtocolsMessage::from_bytes(&data);
}

pub fn fuzz_Mempool_from_bytes(data: &[u8]) {
    let _ = Mempool::from_bytes(&data);
}

pub fn fuzz_MetadataMessage_from_bytes(data: &[u8]) {
    let _ = MetadataMessage::from_bytes(&data);
}

pub fn fuzz_NetworkVersion_from_bytes(data: &[u8]) {
    let _ = NetworkVersion::from_bytes(&data);
}

pub fn fuzz_Operation_from_bytes(data: &[u8]) {
    let _ = Operation::from_bytes(&data);
}

//pub fn OperationHashesForBlock_from_bytes(data: &[u8]) {
//    let _ = OperationHashesForBlock::from_bytes(&data);
//}

//pub fn OperationHashesForBlocksMessage_from_bytes(data: &[u8]) {
//    let _ = OperationHashesForBlocksMessage::from_bytes(&data);
//}

pub fn fuzz_OperationMessage_from_bytes(data: &[u8]) {
    let _ = OperationMessage::from_bytes(&data);
}

pub fn fuzz_OperationsForBlock_from_bytes(data: &[u8]) {
    let _ = OperationsForBlock::from_bytes(&data);
}

pub fn fuzz_OperationsForBlocksMessage_from_bytes(data: &[u8]) {
    let _ = OperationsForBlocksMessage::from_bytes(&data);
}

pub fn fuzz_PeerMessageResponse_from_bytes(data: &[u8]) {
    let _ = PeerMessageResponse::from_bytes(&data);
}

pub fn fuzz_Protocol_from_bytes(data: &[u8]) {
    let _ = Protocol::from_bytes(&data);
}

pub fn fuzz_ProtocolMessage_from_bytes(data: &[u8]) {
    let _ = ProtocolMessage::from_bytes(&data);
}

pub fn fuzz_SwapMessage_from_bytes(data: &[u8]) {
    let _ = SwapMessage::from_bytes(&data);
}
