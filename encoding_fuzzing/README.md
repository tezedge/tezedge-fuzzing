# Tezos Encoding Fuzz Targets

This project contains targets for fuzz testing of functions implementing
Zarith number and MuTez encodings:

- `BinaryReader_read_Z`
- `BinaryReader_read_Mutez`

## Running Individual Target

``` sh
cargo hfuzz run <target>
```

## Running All Targets in Sequence

``` sh
./run_all.sh
```
