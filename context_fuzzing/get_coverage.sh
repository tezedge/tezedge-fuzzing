#!/bin/bash

input_folder="input"
output_folder="cov"

# create output folder
mkdir $output_folder

# get nombre file inside the input folder
nombre_file=$(ls $input_folder | wc | awk '{print $1}')
echo "Number of input files to test: $nombre_file"

count=0

# kcov --include-path .,..,../code/tezedge_seb_storage ./cov ./hfuzz_target/x86_64-unknown-linux-gnu/debug/workingtree_deserialize_serialize_fuzz hfuzz_workspace/workingtree_deserialize_serialize_fuzz/*


# iterate over all file
for i in $(ls $input_folder);
do
	kcov --include-path=.,..,../code/tezedge_seb_storage --verify ./$output_folder/input_$count ./target/debug/context_fuzzing ./$input_folder/$i;
	((count++));
	echo "[++] Count of files processed: $count";
done


# merge all result
kcov --merge ./$output_folder/merged-output ./$output_folder/input_*