# Storage Decoding Fuzz Targets

This project contains fuzz targets for decoding functions defined in the Tezedge
`strorage` module
([link](https://github.com/tezedge/tezedge/tree/master/storage)):

- `BlockHeaderWithHash_decode`
- `ContextActionByBlockHashKey_decode`
- `ContextActionByContractIndexKey_decode`
- `ContextActionByTypeIndexKey_decode`
- `Hash_decode`
- `MempoolKey_decode`
- `Meta_decode`
- `MetaKey_decode`
- `OperationKey_decode`

## Running Individual Target

``` sh
cargo hfuzz run <target>
```

## Running All Targets in Sequence

``` sh
./run_all.sh
```
