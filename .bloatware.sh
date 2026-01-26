install_bloatware(){
    if [ $# -gt 0 ]; then
        local prev_default=$(kubectl config get-contexts | awk '$1 == "*" {print $2}')
        kubectl config use-context $1
    fi

    kubectl create namespace prometheus
    helm install prometheus-community/kube-prometheus-stack \
        --generate-name \
        --namespace prometheus

    kubectl create namespace argo-rollouts
    kubectl apply -n argo-rollouts \
        -f https://github.com/argoproj/argo-rollouts/releases/latest/download/install.yaml
    
    if [ $# -gt 0 ]; then
        kubectl config use-context $prev_default
    fi
}