# tezedge_fuzz

`tezedge_fuzz` is a wrapping library around interesting tezedge methods to fuzz. This library can be used to easily interface with honggfuzz, cargo-fuzz and other tools.

# Makefile commands

## list

List all the fuzzing targets. 

``` sh
make list
```

## run

``` sh
# run fuzzing for one target
make a=AdvertiseMessage_from_bytes
```

## dry-run 

This mode will run the complete corpora into the fuzzing target.

``` sh
# dry-run only one target
make dry_run a=AdvertiseMessage_from_bytes
# dry-run all
make dry_run_all
```