# API Reference for warp-prometheus

`warp-prometheus` is a Rust library designed to seamlessly integrate Prometheus metrics into applications built with the Warp web framework. It provides a straightforward mechanism to capture and expose HTTP request durations, methods, paths, and status codes, enabling real-time monitoring of service performance.

## Architecture

The core of `warp-prometheus` is the `Metrics` struct. This struct is initialized with a Prometheus `Registry` and a list of specific path segments (`include_path_labels`) that should be preserved in metric labels, rather than being anonymized. Upon instantiation, `Metrics` registers a `HistogramVec` named `server_response_duration_seconds` with the provided registry. This histogram uses two labels: `classifier` (combining the HTTP method and a sanitized request path) and `status` (the HTTP response status code).

The `http_metrics` method serves as the primary entry point for metric collection. It is designed to be used with Warp's `warp::log::custom` filter. When invoked, it extracts request information (method, path, status, elapsed time) from the `warp::log::Info` struct. The request path is then processed by the `sanitize_path_segments` helper function. This function transforms dynamic path segments (e.g., user IDs) into wildcards (`*`), while preserving specific, configured path segments (like `/users` or `/registration`), ensuring consistent metric categorization regardless of specific resource IDs. Finally, the sanitized path, method, and status are used to observe the request duration in the registered Prometheus histogram.

## Key Files

*   **`Cargo.toml`**: Defines the project's metadata, authors, description, license, and specifies its dependencies, including `warp` (the web framework), `prometheus` (for metrics), and `log` (for logging).
*   **`src/lib.rs`**: Contains the full implementation of the `Metrics` struct. This includes its `new` constructor, the `http_metrics` function responsible for metric collection, and the `sanitize_path_segments` helper method that handles path anonymization. It also hosts unit tests for the path sanitization logic.
*   **`README.md`**: Provides a high-level overview of the library, its purpose, project badges (docs, license, CI status), and a concise example demonstrating how to set up and integrate `warp-prometheus` into a Warp application.

## How to Run

As `warp-prometheus` is a library, it is integrated into your existing Warp application rather than run standalone.

1.  **Add to `Cargo.toml`**:
    Add `warp-prometheus` as a dependency in your project's `Cargo.toml`:
    ```toml
    [dependencies]
    warp-prometheus = "0.5.0"
    warp = "0.3.0"
    prometheus = "0.13.0"
    log = "0.4.0"
    ```
    *Note: Ensure `warp`, `prometheus`, and `log` versions are compatible.*

2.  **Integrate with your Warp application**:
    Initialize the Prometheus registry and `Metrics` struct, then apply the `http_metrics` function using `warp::log::custom` to your routes.

    ```rust
    use prometheus::Registry;
    use warp_prometheus::Metrics;
    use warp::Filter;

    #[tokio::main] // If using a modern async runtime
    async fn main() {
        let registry: Registry = Registry::new();
        // Define path segments that should NOT be replaced by '*'
        let path_includes: Vec<String> = vec![String::from("hello"), String::from("metrics")];

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

        // Example route to expose Prometheus metrics (optional, but common)
        let metrics_route = warp::path("metrics").map(move || {
            use prometheus::Encoder;
            let encoder = prometheus::TextEncoder::new();
            let metric_families = registry.gather(); // Use the same registry
            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        });

        let routes = test_routes.or(metrics_route);

        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
    ```

## How to Test

Unit tests for `warp-prometheus` are embedded within `src/lib.rs` inside a `#[cfg(test)]` module. These tests primarily verify the correctness of the `sanitize_path_segments` function, ensuring that path segments are appropriately anonymized or preserved based on the configured `include_path_labels`.

To execute the tests, navigate to the project root directory and run:

```bash
cargo test
```