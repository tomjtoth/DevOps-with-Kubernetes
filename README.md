# DevOps with Kubernetes

Tämä on palautusrepo HY:n kurssille.

## Tags

Docker images ([`tomjtoth/devops-with-kubernetes:service-x.y`](https://hub.docker.com/r/tomjtoth/devops-with-kubernetes/tags)) are generated via [GHA](.github/workflows/deploy.yml#L19-L37), tags are parsed from the HEAD commit message ("... `#deploy <PROD> service(OPTIONAL_PLACEHOLDER):x.y` ...") via [regex](https://regex101.com/r/H4BBVD).

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
- [2.4](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.4/todo_app)
- [2.5](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.5/log_server)
- [2.6](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.6/todo_app)
- [2.7](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.7/pingpong)
- [2.8](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.8/todo_backend)
- [2.9](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.9/todo_reminder)
- [2.10](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/2.10/todo_backend)

### Chapter 4

- [3.1](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.1/pingpong)
- [3.2](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.2/log_output)
- [3.3](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.3/log_output)
- [3.4](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.4/log_output)
- [3.5](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.5/todo_app)
- 3.6 **SKIPPED**
- 3.7 **SKIPPED**
- 3.8 **SKIPPED**
- [3.9](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.9/todo_app)
- 3.10 **SKIPPED**
- [3.11](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/3.11/todo_app)
- 3.12 **PENDING**: should be easy, but logs are not visible in the browser

### Chapter 5

- [4.1](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.1/pingpong)
- [4.2](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.2/todo_app)
- [4.3](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.3)
  got 2 results:
  ```sh
  kube_pod_info{namespace="prometheus", created_by_kind="StatefulSet"}
  ```
  ![pods created by statefulSet in ns prometheus](prometheus-query-res1.png)
- [4.4](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.4/pingpong)
- [4.5](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.5/todo_app)
  no time for migrations atm, I teardown/re-create my clusters often enough, anyways 🤓
- 4.6 **SKIPPED**
- [4.7](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.7/.github/workflows/deploy.yml)
  deployment only via my original script (see above) was [successful](https://github.com/tomjtoth/DevOps-with-Kubernetes/actions/runs/21367094242)
- [4.8](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.8/.github/workflows/deploy.yml)
  deployment only via my original script (see above) was [successful](https://github.com/tomjtoth/DevOps-with-Kubernetes/actions/runs/21367739961)

- [4.9](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.9/.github/workflows/deploy.yml)
  Tweaked my original deployment workflow to this step, which resulted in a more complex logic, so decided to keep it instead
  - The below is triggered via HEAD commit messages, such as `#deploy todo-app(APP):4.9`, logs of successful run [here](https://github.com/tomjtoth/DevOps-with-Kubernetes/actions/runs/21444924003/job/61757877456)

    > Each commit to the main branch should result in deployment to the staging environment

  - The below gets trigggered via HEAD commit messages, like: `#deploy PROD todo-app(APP):4.9`, details [here](.github/workflows/deploy.yml#L88-L98), successful run [here](https://github.com/tomjtoth/DevOps-with-Kubernetes/actions/runs/21444707730/job/61757092527)

    > Each tagged commit results in deployment to the production environment

  - And the below tasks were skipped previously, so this task is actually much simpler 😎
    > In staging the broadcaster just logs all the messages, it does not forward those to any external service
    > In staging database is not backed up

- [4.10](https://github.com/tomjtoth/DevOps-with-Kubernetes/tree/4.10/todo_app)
  Manifests and cluster helper scripts were moved [here](https://github.com/tomjtoth/dwk-conf).
  This disables the automated release process established in the prev task 😭
