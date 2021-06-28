# Tezos Messages Fuzz Targets

This project contains fuzz targets for testing
`BinaryRead::from_bytes()`
([link](https://github.com/tezedge/tezedge/blob/master/tezos/messages/src/p2p/binary_message.rs#L28-L32))
method implementation provided by the message types defined
[here](https://github.com/tezedge/tezedge/tree/master/tezos/messages/src/p2p/encoding).

Here is the list of fuzz targets:

- `AckMessage_from_bytes`
- `AdvertiseMessage_from_bytes`
- `BinaryChunk_from_content`
- `BlockHeader_from_bytes`
- `BlockHeaderMessage_from_bytes`
- `Component_from_bytes`
- `ConnectionMessage_from_bytes`
- `CurrentBranch_from_bytes`
- `CurrentBranchMessage_from_bytes`
- `CurrentHeadMessage_from_bytes`
- `DeactivateMessage_from_bytes`
- `GetBlockHeadersMessage_from_bytes`
- `GetCurrentBranchMessage_from_bytes`
- `GetCurrentHeadMessage_from_bytes`
- `GetOperationsForBlocksMessage_from_bytes`
- `GetOperationsMessage_from_bytes`
- `GetProtocolsMessage_from_bytes`
- `Mempool_from_bytes`
- `MetadataMessage_from_bytes`
- `NetworkVersion_from_bytes`
- `Operation_from_bytes`
- `OperationMessage_from_bytes`
- `OperationsForBlock_from_bytes`
- `OperationsForBlocksMessage_from_bytes`
- `PeerMessageResponse_from_bytes`
- `Protocol_from_bytes`
- `ProtocolMessage_from_bytes`
- `SwapMessage_from_bytes`

## Running Individual Target

``` sh
cargo hfuzz run <target>
```

## Running All Targets in Sequence

``` sh
./run_all.sh
```
