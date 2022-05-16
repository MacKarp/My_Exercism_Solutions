#!/usr/bin/env bash

main() {
    checkInput "$@"
    calculateDistance "$@"
}

checkInput() {
    checkInputCount "$@"
    checkInputLength "$@"
}

checkInputCount() {
    if [ "$#" -ne 2 ]; then
        echo "Usage: hamming.sh <string1> <string2>"
        exit 1
    fi
}

checkInputLength() {
    input1=$1
    input2=$2
    if [ "${#input1}" -ne "${#input2}" ]; then
        echo "strands must be of equal length"
        exit 1
    fi
}

calculateDistance() {
    input1=$1
    input2=$2
    distance=0

    for ((i = 0; i < ${#input1}; ++i)); do
        if [ "${input1:$i:1}" != "${input2:$i:1}" ]; then
            (("distance++")) || true
        fi
    done

    echo "$distance"
}

main "$@"
