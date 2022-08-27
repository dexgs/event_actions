#!/bin/bash
cd `dirname $0`

make -sC actions
cargo -q run --release -- actions/lib/obj.so
