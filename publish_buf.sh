#!/bin/sh

set -euxo pipefail

ITER=${1:-1}

buildImage() {
    PLUGIN=$1
    IMAGE=$2
    VERSION=$(cargo read-manifest --manifest-path "protoc-gen-$PLUGIN/Cargo.toml" 2>/dev/null | jq -r .version)

    docker buildx build --platform=linux/amd64 --build-arg "BIN=protoc-gen-$PLUGIN" -t "plugins.buf.build/prost/$IMAGE:$VERSION-$ITER" --push .
}

buildImage prost prost
buildImage prost-serde serde
buildImage prost-crate crate
buildImage tonic tonic
