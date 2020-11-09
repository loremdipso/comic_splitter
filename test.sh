#!/bin/bash

cd ./test
rm -rf ./output
mkdir ./output &>/dev/null
mkdir ./output/png &>/dev/null
mkdir ./output/jpg &>/dev/null

set -e
time cargo run --release -q ./input/*.jpg --output ./output/jpg
time cargo run --release -q ./input/*.png --output ./output/png

