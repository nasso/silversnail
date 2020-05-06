#!/bin/bash

PLUGINS_SRC="$(realpath ./plugins/*)"
PLUGIN_DIR="$(realpath ./host-native/plugins)"

mkdir -p $PLUGIN_DIR

function build_plug() {
  cd $1
  cargo build --release
  cp ./target/wasm32-unknown-unknown/release/*.wasm $PLUGIN_DIR
}

for plug in $PLUGINS_SRC; do
  build_plug "$plug"
done

for plug in $PLUGIN_DIR/*.wasm; do
  wasm-gc $plug
done
