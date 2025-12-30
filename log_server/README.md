# Log server

You probably only need `rustc` or `rustup` for this. Then you can run it with

```sh
LOG_PATH=../log_output/data/log \
cargo run
```

..make sure you're running both [log_output](../log_output) and [pingpong](../pingpong) first in the background.

...or deploy everything via

```sh
kubectl apply -f ../pingpong/manifests
kubectl apply -f ../log_output/manifests
```
