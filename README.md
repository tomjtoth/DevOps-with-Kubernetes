# DevOps with Kubernetes

Tämä on palautusrepo HY:n kurssille.

## Tags

Docker images ([`tomjtoth/devops-with-kubernetes:service-x.y`](https://hub.docker.com/r/tomjtoth/devops-with-kubernetes/tags)) are generated via [GHA](.github/workflows/deploy.yml#L19-L30), tags are parsed from the HEAD commit message ("... `#deploy service:x.y` ...") via regex.

## K3s tweaks

Recreating the cluster

```sh
k3d cluster delete
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube
for app in ns/manifests pv/manifests */manifests; do kubectl apply -f $app; done
```

### Chapter 2

- [1.1](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.1/log_output)
- [1.2](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.2/project)
- [1.3](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.3/log_output)
- [1.4](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.4/todo_app)
- [1.5](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.5/todo_app)
- [1.6](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.6/todo_app)
- [1.7](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.7/log_output)
- [1.8](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.8/todo_app)
- [1.9](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.9/pingpong)
- 1.10
  - [output](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.10/log_output)
  - [server](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.10/log_server)
- [1.11](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.11/log_server)
- [1.12](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.12/todo_app)
- [1.13](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/1.13/todo_app)

### Chapter 3

- 2.1
  - [pingpong](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.1/pingpong)
  - [log_server](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.1/log_server)
- 2.2
  - [todo-app](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.2/todo_app)
  - [todo-backend](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.2/todo_backend)
- [2.3](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.3/log_output)
