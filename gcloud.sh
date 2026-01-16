#!/bin/bash

cd "$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
source .parser.sh

gke_op() {
    local op=$1
    shift

    gcloud container clusters $op dwk-cluster \
        --zone=europe-north1-b \
        "$@"
}

if [ -v DELETE ]; then
    gke_op delete
fi

if [ -v RESIZE ]; then
    gke_op resize \
        --node-pool=default-pool \
        --num-nodes=$RESIZE
fi

if [ -v CREATE ]; then
    gke_op create \
        --cluster-version=1.32 \
        --disk-size=32 \
        --num-nodes=3 \
        --machine-type=e2-micro
fi


gke(){
    echo kubectl $1 -f "$2"
}

if [ -v DELETE_ALL ]; then
    DELETE_THESE=({!(ns|pv),pv,ns}/manifests/{{ing,{svc,service}}*,!(ing*|svc*|service*)}.yml)
fi

if [ -v APPLY_ALL ]; then
    APPLY_THESE=({ns,pv,!(ns|pv)}/manifests/*.yml)
fi

has_gke_version(){
    local gke_manifest="${1%.yml}.gke.yml"
    shift

    if [ -f "$gke_manifest" ] && [[ " $@ " == *" $gke_manifest "* ]]; then
        return 0
    fi

    return 1
}

if [ -v DELETE_THESE ]; then
    for manifest in "${DELETE_THESE[@]}"; do
        has_gke_version "$manifest" "${DELETE_THESE[@]}" && \
            continue
        
        gke delete "$manifest"
    done
fi

if [ -v APPLY_THESE ]; then
    for manifest in "${APPLY_THESE[@]}"; do
        has_gke_version "$manifest" "${APPLY_THESE[@]}" && \
            continue
        
        gke apply "$manifest"
    done
fi

stop_timer
