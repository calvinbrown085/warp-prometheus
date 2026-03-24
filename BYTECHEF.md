# BYTECHEF.md

## What this project does

`warp-prometheus` is a Rust library designed to seamlessly integrate Prometheus metrics into applications built using the `warp` web framework. It provides a `Metrics` struct that, when used with `warp::log::custom`, automatically collects HTTP request duration and status, and intelligently sanitizes request paths to generate consistent and aggregated Prometheus labels.

## Architecture

The library's core is the `Metrics` struct, which encapsulates a Prometheus `HistogramVec` for tracking `server_response_duration_seconds`. It integrates with `warp` applications by providing the `http_metrics` method, designed to be used with the `warp::log::custom` filter. This method records request duration, HTTP method, and status code. A key architectural feature is the `sanitize_path_segments` method, which transforms dynamic path segments (e.g., IDs) into wildcards (`*`) to ensure consistent and manageable Prometheus labels for route classification, based on a configurable list of included path segments.

## Key files

*   **`Cargo.toml`**: Defines the Rust package, its metadata (name, version, description, license), and its direct dependencies: `warp` for the web framework, `prometheus` for the metrics client library, and `log` for internal logging.
*   **`README.md`**: Provides a high-level overview of the library, including project badges (docs, license, CI status) and a quick example demonstrating how to initialize and apply the `Metrics` struct to a `warp` route.
*   **`src/lib.rs`**: Contains the complete implementation of the `warp-prometheus` library. This includes the `Metrics` struct definition, its constructor (`new`), the internal `sanitize_path_segments` logic for path anonymization, and the public `http_metrics` function which integrates with `warp`'s logging to record Prometheus HTTP metrics. The file also contains unit tests for the path sanitization logic.

## How to run

This project is a Rust library and is not run as a standalone application. To use it, integrate it into your `warp` application:

1.  Add `warp-prometheus` to your `Cargo.toml` dependencies.
2.  Initialize a `prometheus::Registry` (or use `prometheus::default_registry()`).
3.  Create an instance of `warp_prometheus::Metrics`, providing the registry and a `Vec<String>` of path segments you wish to explicitly include in your metric labels (other segments will be wildcarded).
4.  Apply the `metrics.http_metrics` function using `warp::log::custom` to your desired `warp` filters, as shown in the examples in `README.md` and `src/lib.rs`.
5.  Ensure your application exposes the Prometheus registry's metrics endpoint (e.g., `/metrics`) for scraping.

Example usage within a `warp` application:

```rust
use prometheus::Registry;
use warp_prometheus::Metrics;
use warp::Filter;

// Initialize Prometheus registry and Metrics instance
let registry: Registry = Registry::new();
let path_includes: Vec<String> = vec![String::from("hello")];
let metrics = Metrics::new(&registry, &path_includes);

// Define your warp route
let route_one = warp::path("hello")
    .and(warp::path::param())
    .and(warp::header("user-agent"))
    .map(|param: String, agent: String| {
        format!("Hello {}, whose agent is {}", param, agent)
    });

// Apply the metrics filter to your routes
let test_routes = route_one.with(warp::log::custom(move |log| {
    metrics.http_metrics(log)
}));

// In a real application, you would then serve `test_routes`
// and also expose a /metrics endpoint from `registry`.
// For example:
// let routes = test_routes.or(warp_prometheus::metrics_route(&registry));
// warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
```

## How to test

To run the project's unit tests, use the Cargo test command:

```bash
cargo test
```