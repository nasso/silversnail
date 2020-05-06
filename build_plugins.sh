#!/bin/bash

PLUGINS_SRC="$(realpath ./plugins/*)"
NATIVE_PLUGINS="$(realpath ./host-native/plugins)"
WEB_PLUGINS="$(realpath ./host-web/plugins)"

mkdir -p $NATIVE_PLUGINS
mkdir -p $WEB_PLUGINS

function build_plug() {
  cd $1
  cargo build --release
  cp ./target/wasm32-unknown-unknown/release/*.wasm $NATIVE_PLUGINS
}

for plug in $PLUGINS_SRC; do
  build_plug "$plug"
done

for plug in $NATIVE_PLUGINS/*.wasm; do
  wasm-gc $plug
done

cp $NATIVE_PLUGINS/*.wasm $WEB_PLUGINS
