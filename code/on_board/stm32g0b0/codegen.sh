#!/bin/sh

patch_yaml="stm32g0b0.yaml"
patched_svd="stm32g0b0_patched.svd"

# blow up at first problem
set -e

# clean src subdirectory
rm -rf src/
mkdir src/

# patch SVD
svdtools patch "$patch_yaml" "$patched_svd"

# turn into Rust code
svd2rust \
    --strict \
    --pascal_enum_values \
    --max_cluster_size \
    --atomics \
    --atomics_feature atomics \
    -i "$patched_svd"

# break into modules
form \
    -i "lib.rs" \
    -o "src"

# beautify
cargo fmt
