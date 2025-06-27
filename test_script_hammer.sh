#!/bin/sh
for i in $(seq 1 100); do
    echo "Running test iteration $i"
    cargo test
    if [ $? -ne 0 ]; then
        echo "Test failed on iteration $i"
        exit 1
    fi
done
