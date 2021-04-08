# warp-prometheus

Afterthought of Prometheus metrics for [Warp](https://github.com/seanmonstar/warp)
[![Docs](https://docs.rs/warp-prometheus/badge.svg)](https://docs.rs/warp-prometheus/)
[![Apache-2 licensed](https://img.shields.io/crates/l/warp-prometheus.svg)](./LICENSE)
[![CI](https://github.com/calvinbrown085/warp-prometheus/workflows/Rust/badge.svg)](https://github.com/calvinbrown085/warp-prometheus/actions?query=workflow%3ARust)


## Example
```rust
use prometheus::Registry;
use warp_prometheus::Metrics;
use warp::Filter;

let registry: Registry = Registry::new();
let path_includes: Vec<String> = vec![String::from("hello")];

let route_one = warp::path("hello")
    .and(warp::path::param())
    .and(warp::header("user-agent"))
    .map(|param: String, agent: String| {
    format!("Hello {}, whose agent is {}", param, agent)
    });

    let metrics = Metrics::new(&registry, &path_includes);

    let test_routes = route_one.with(warp::log::custom(move |log| {
        metrics.http_metrics(log)
    }));
```
