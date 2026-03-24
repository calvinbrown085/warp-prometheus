# warp-prometheus: Prometheus Metrics for Warp Applications

`warp-prometheus` is a Rust library designed to provide "afterthought" Prometheus metrics for applications built with the `warp` web framework. It simplifies the collection of HTTP request duration and status metrics, offering a configurable mechanism to sanitize URL paths for consistent metric labeling and aggregation.

## Architecture
The core of this library is the `Metrics` struct. Upon instantiation using `Metrics::new`, it registers a `HistogramVec` with a Prometheus `Registry`. This `HistogramVec` is named `server_response_duration_seconds` and tracks HTTP request durations, categorized by two labels: `classifier` (combining HTTP method and a sanitized path) and `status` (the HTTP response status code). The `Metrics` struct integrates with `warp` applications via the `warp::log::custom` filter, where its `http_metrics` method processes request information (`warp::log::Info`). A key component is the private `sanitize_path_segments` method, which anonymizes dynamic parts of request paths (e.g., IDs) by replacing them with `*`, based on a user-provided list of path segments (`include_path_labels`) that should *not* be anonymized.

## Key files

*   **`Cargo.toml`**: Defines the project as `warp-prometheus`, specifies its metadata (version, authors, description, license), and lists its primary dependencies: `log` for logging, `prometheus` for metrics integration, and `warp` as the web framework it extends.
*   **`README.md`**: Provides a high-level overview of the library, including its purpose, license information, CI status, and a minimal Rust code example demonstrating how to integrate `warp-prometheus` into a `warp` application.
*   **`src/lib.rs`**: Contains the complete implementation of the `warp-prometheus` library. This file defines the `Metrics` struct, its public constructor `Metrics::new`, the core `http_metrics` method responsible for recording Prometheus metrics from `warp` logs, and the `sanitize_path_segments` helper function for processing URL paths. It also includes unit tests for the path sanitization logic.

## How to run
`warp-prometheus` is a library and is integrated into your existing `warp` application.

1.  **Add to `Cargo.toml`**: Include `warp-prometheus` as a dependency in your project's `Cargo.toml` file:
    ```toml
    [dependencies]
    warp-prometheus = "0.5.0" # Use the latest version
    prometheus = "0.13.0"    # Ensure prometheus is also added if not already
    warp = "0.3.0"           # Ensure warp is also added if not already
    log = "0.4.0"            # Ensure log is also added if not already
    ```

2.  **Integrate with your `warp` routes**:
    Instantiate the `Metrics` struct, providing a Prometheus `Registry` and a list of `include_path_labels`. These labels are specific path segments (e.g., "users", "api") that you want to preserve in your metrics, while other dynamic segments (like user IDs) will be replaced with `*`. Then, apply the `metrics.http_metrics` method using `warp::log::custom` to your routes.

    ```rust
    use prometheus::{Registry, TextEncoder, Encoder};
    use warp_prometheus::Metrics;
    use warp::Filter;

    #[tokio::main]
    async fn main() {
        // Initialize a Prometheus Registry
        let registry: Registry = Registry::new();
        // Define path segments to include in metric labels (others will be anonymized to '*')
        let path_includes: Vec<String> = vec![String::from("hello"), String::from("users")];

        // Initialize the Metrics handler
        let metrics_handler = Metrics::new(&registry, &path_includes);

        // Your application routes
        let hello_route = warp::path("hello")
            .and(warp::path::param())
            .map(|param: String| {
                format!("Hello, {}!", param)
            });

        let users_route = warp::path("users")
            .and(warp::path::param())
            .map(|id: u32| {
                format!("User ID: {}", id)
            });

        // Combine routes and apply the metrics filter
        let routes = hello_route
            .or(users_route)
            .with(warp::log::custom(move |info| {
                metrics_handler.http_metrics(info);
            }));

        // --- Exposing Metrics Endpoint ---
        // This library *collects* metrics. You'll need to expose them
        // yourself, typically on a `/metrics` endpoint.
        let metrics_registry = registry.clone(); // Clone for the metrics endpoint filter
        let metrics_route = warp::path("metrics")
            .map(move || {
                let mut buffer = Vec::new();
                let encoder = TextEncoder::new();
                let metric_families = metrics_registry.gather();
                encoder.encode(&metric_families, &mut buffer).unwrap();
                String::from_utf8(buffer).unwrap()
            });

        // Combine application routes with the metrics exposure route
        let final_routes = routes.or(metrics_route);

        warp::serve(final_routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
    ```
    *Note*: The example above provides a complete basic Warp server including a `/metrics` endpoint for demonstration. The `warp-prometheus` library itself *only* handles the metric collection logic, not the exposition of the `/metrics` endpoint.

## How to test
To run the included unit tests, navigate to the project root and execute:
```bash
cargo test
```
The tests primarily verify the `sanitize_path_segments` function's behavior across various URL path structures, ensuring that dynamic segments are correctly anonymized while specified segments are preserved.