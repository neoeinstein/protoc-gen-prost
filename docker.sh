#!/bin/sh

PLUGIN=$1
IMAGE=$2

docker buildx build --platform=linux/amd64 --build-arg "BIN=protoc-gen-$PLUGIN" -t "plugins.buf.build/prost/$IMAGE" --push .