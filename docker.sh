#!/bin/sh

PLUGIN=$1
IMAGE=$2

docker build -t "plugins.buf.build/prost/$IMAGE" --build-arg "BIN=protoc-gen-$PLUGIN" . && \
docker push "plugins.buf.build/prost/$IMAGE"
