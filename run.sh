#!/bin/bash
cd `dirname $0`

cargo run --release -- actions/lib/obj.so
