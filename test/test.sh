#!/bin/sh

# Add missing import
export LD_LIBRARY_PATH="../target/release:../target/debug"
# MacOS
export DYLD_LIBRARY_PATH="../target/debug"

# Compile the C test file
gcc test.c -L ../target/release -L ../target/debug -lsyntect_c -o test

./test
