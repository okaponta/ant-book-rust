#!/bin/bash

function test() {
    echo "test for $1"
    command `(cargo run --bin $1 < ./test/in/$1.txt) > ./test/output/$1.txt`
    echo "---OUTPUT---"
    cat ./test/output/$1.txt
    echo "---RESULT---"
    diff -s ./test/out/$1.txt ./test/output/$1.txt > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "\033[32m\033[1mOK\033[m\033[m"
    elif [ $? -eq 1 ]; then
        echo "\033[31m\033[1mNG\033[m\033[m"
        echo "---EXPECT---"
        cat ./test/out/$1.txt
    fi
}
