#/bin/bash

cd "$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
source .parser.sh

if [ -v DELETE ]; then
    k3d cluster delete
fi

if [ -v CREATE ]; then
    k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2

    install_bloatware k3d-k3s-default
fi

k3s(){
    kubectl $1 --context k3d-k3s-default -f "$2"
}

if [ -v DELETE_ALL ]; then
    DELETE_THESE=({!(ns|pv),pv,ns}/manifests/{{ing,{svc,service}}*,!(ing*|svc*|service*)}.yml)
fi

if [ -v APPLY_ALL ]; then
    APPLY_THESE=({ns,pv,!(ns|pv)}/manifests/!(*.gke).yml)
fi

if [ -v DELETE_THESE ]; then
    for manifest in "${DELETE_THESE[@]}"; do
        if [[ "$manifest" =~ \.gke\.yml$ ]]; then
            continue
        fi
        k3s delete "$manifest"
    done
fi

if [ -v APPLY_THESE ]; then
    for manifest in "${APPLY_THESE[@]}"; do
        k3s apply "$manifest"
    done
fi

stop_timer
