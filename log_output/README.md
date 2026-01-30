# Exercises namespace

Just reset your k3d cluster, install istio and [deploy](./kustomization.yml) via the below command

```sh
kubectl apply -k .
```

Exposed endpoints reachable at the loadbalancer's external IP created by my-gateway-istio:

- `/pingpong`
- `/pings`
- `/`
