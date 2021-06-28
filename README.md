# tezedge-fuzzing

This repository contains targets for fuzz testing of [Tezedge
node](https://github.com/tezedge/tezedge). Targets are grouped to projects with
relation to Tezedge components.

- [Tezos Encoding](encoding_fuzzing/README.md)
- [Tezos Messages](tezos_messages_fuzzing/README.md)
- [Crypto](crypto_fuzzing/README.md)
- [Storage](storage_fuzzing/RE)


## Requirements

Rust toolchain is needed to run fuzz targets. Currently `tezedge` works with
`nightly-2020-31-12` toolchain. It can be set up as the default toolchain using
the following command:

``` sh
rustup default nightly-2020-31-12
```

You will need `cargo-hfuzz` to be able to run fuzz targets. It can be installed
using the following command:

``` sh
cargo install hfuzz
```

Before running any fuzz target you need to make sure that the `tezedge`
repository is in synch with required version (e.g. `master` or `develop`).

If you want to fuzz lates development version you can do the following:

``` sh
cd code/tezedge
git checkout develop
git pull --rebase
```

## Running Fuzz Targets

Each fuzz target can be run individually. E.g. to run fuzzing for
`PeerMessageResponse_from_bytes` target from `tezos_messages_fuzzing` project,
do the following:

``` sh
cd tezos_messages_fuzzing
cargo hfuzz run PeerMessageResponse_from_bytes
```

To specify any additional parameters to the `honggfuzz` use the `HFUZZ_RUN_ARGS`
environment variable (see
[here](https://github.com/google/honggfuzz/blob/master/docs/USAGE.md#cmdline---help)):


``` sh
HFUZZ_RUN_ARGS="--timeout 1 --threads 16" cargo hfuzz run PeerMessageResponse_from_bytes
```

It is also possible to run each target from a project for 4_000_000 iterations:

``` sh
cd tezos_messages_fuzzing
./run_all.sh
```

