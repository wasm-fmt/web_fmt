#!/bin/bash

cd $(dirname $0)/..
crates_dir=$(pwd)

# Default to script-biome if no features specified
FEATURES="${1:-script-biome}"

# Validate features for web_fmt (only script-biome or script-oxc)
case "$FEATURES" in
  script-biome)
    BUILD_ARGS="--features $FEATURES"
    ;;
  script-oxc)
    BUILD_ARGS="--no-default-features --features $FEATURES"
    ;;
  *)
    echo "Error: Invalid feature '$FEATURES'"
    echo "Usage: ./build.sh [script-biome|script-oxc]"
    echo "  script-biome: Build with Biome formatter (default)"
    echo "  script-oxc:   Build with OXC formatter (experimental with embedded CSS)"
    exit 1
    ;;
esac

# Build with wasm-pack from the crates/web_fmt directory
wasm-pack build --target=web --scope=wasm-fmt $BUILD_ARGS

cp -R ./extra/. ./pkg/

./scripts/package.mjs ./pkg/package.json
