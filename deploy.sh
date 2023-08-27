#!/bin/bash
# shellcheck disable=SC2181

echo "cargo clean"
cargo clean
if [ $? -ne 0 ]; then
    echo "cargo clean failed. Exiting..."
    exit 1
fi

#----------------------------------------------------------------------
echo ""
echo "cargo build"
cargo build \
--target wasm32-unknown-unknown \
--release
if [ $? -ne 0 ]; then
    echo "cargo build failed. Exiting..."
    exit 1
fi

#----------------------------------------------------------------------
echo ""
echo "soroban contract deploy"
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/cowchain_farm.wasm \
--rpc-url https://rpc-futurenet.stellar.org:443 \
--network-passphrase 'Test SDF Future Network ; October 2022'
if [ $? -ne 0 ]; then
    echo "soroban contract deploy failed. Exiting..."
    exit 1
fi