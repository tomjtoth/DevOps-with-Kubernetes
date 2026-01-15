#!/bin/bash

USAGE="proper usage is: $(basename $0) OPTIONS

where OPTIONS are:
--delete        teardown the cluster
--create        redefine the cluster
--reset         teardown + redefine the cluster
--resize N      scale number of nodes in the pool
"

if [ $# -eq 0 ]; then
    echo "$USAGE"
    exit 1
fi

TAB=$(printf '\t')

while [ $# -gt 0 ]; do
    case "$1" in 
        (--delete) DELETING=1;;

        (--create) CREATING=1;;

        (--reset) DELETING=1 CREATING=1;;

        (--resize)
            if [[ ! "$2" =~ ^[0-9]+$ ]]; then
                UNKNOWN_FLAGS+=("$TAB$1 NNN <- needs to be a numeric arg, found \"$2\"")
            else
                RESIZE=$2
            fi
            shift
            ;;

        (*) UNKNOWN_FLAGS+=("$TAB$1");;
    esac
    shift
done

if [ -v UNKNOWN_FLAGS ]; then 
    printf '%s\n' \
        "found the following unknown flags:" \
        "${UNKNOWN_FLAGS[@]}" \
        "" \
        "$USAGE"
    exit 1
fi

op() {
    local op=$1
    shift

    gcloud container clusters $op dwk-cluster \
        --zone=europe-north1-b \
        "$@"
}

START_TIME=$(date +%s)

if [ -v DELETING ]; then
    op delete
fi

if [ -v RESIZE ]; then
    op resize \
        --node-pool=default-pool \
        --num-nodes=$RESIZE
fi

if [ -v CREATING ]; then
    op create \
        --cluster-version=1.32 \
        --disk-size=32 \
        --num-nodes=3 \
        --machine-type=e2-micro
fi

echo "operation took $(($(date +%s) - $START_TIME)) seconds to finish"
