#/bin/bash

USAGE="proper usage is: $(basename $0) [OPTIONS]

where OPTIONS are:
--reset     teardown + redefine the cluster
"

while [ $# -gt 0 ]; do
    case "$1" in 
        (--reset) RESETTING=1;;
        (*) UNKNOWN_FLAGS+=("$1")
    esac
    shift
done

if [ -v UNKNOWN_FLAGS ]; then
    printf '%s\n' \
        "found the following unknown flags: ${UNKNOWN_FLAGS[*]}" \
        "" \
        "$USAGE"
    exit 1
fi

if [ -v RESETTING ]; then
    k3d cluster delete
    k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2

    docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube-{exercises,project}
fi

root_dir="$(dirname "${BASH_SOURCE[0]}")"
cd "$root_dir"

for manifests in {ns,pv,*}/manifests; do
    if [[ "$manifests" =~ \.gke\.yml$ ]]; then
        continue
    fi
    kubectl --context k3d-k3s-default apply -f $manifests
done
