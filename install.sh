#!/usr/bin/env bash

set -e
set -x

cargo build -r

sudo cp -pv target/release/trashword /usr/local/bin/trashword
