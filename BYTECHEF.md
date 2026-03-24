# BYTECHEF.md

## Project Description
`warp-prometheus` is a Rust library designed to provide "afterthought" Prometheus metrics for web services built with the `warp` framework. It integrates by observing `warp`'s log events, extracting request details, and recording them as Prometheus histograms, with a focus on normalizing route paths to manage metric cardinality.

## Architecture
The core of this library is the `Metrics` struct. It is initialized with a Prometheus `Registry` and a `Vec<String>` of path segments to include as labels. During initialization, it registers a `HistogramVec` named `server_response_duration_seconds` to track route response times, labeled by `classifier` (a combination of HTTP method and a sanitized path) and `status` (HTTP status code).

The `Metrics::http_metrics` method serves as the integration point with `warp`'s custom logging. It receives `warp::log::Info` for each request, which contains details like method, path, status, and elapsed time. Before recording, the request path is processed by `sanitize_path_segments`. This method replaces non-specified path segments with `*` (e.g., `/users/123` becomes `/users/*` if "users" is an included path) to aggregate metrics across similar routes and reduce label cardinality. Finally, the processed information is used to observe the request duration in the `http_timer` histogram.

## Key Files
*   **`Cargo.toml`**: Defines the project as `warp-prometheus`, specifies its `0.5.0` version, authorship, Apache-2.0 license, and key dependencies: `log` for logging, `prometheus` for metrics, and `warp` for web framework integration.
*   **`README.md`**: Provides a high-level overview of the library's purpose and includes a basic example demonstrating how to initialize `Metrics` and integrate it with `warp` routes using `warp::log::custom`.
*   **`src/lib.rs`**: Contains the complete implementation of the `Metrics` struct.
    *   `Metrics::new`: Constructor that initializes and registers the Prometheus `HistogramVec`.
    *   `Metrics::sanitize_path_segments`: A private helper function responsible for normalizing request paths by replacing dynamic segments with wildcards, based on configured `include_path_labels`.
    *   `Metrics::http_metrics`: The public API method that integrates with `warp`'s logging to collect and record HTTP request metrics into the Prometheus histogram.
    *   `mod test`: Contains unit tests for the `sanitize_path_segments` logic, ensuring correct path normalization.
*   **`.github/workflows/rust.yml`**: Defines the Continuous Integration (CI) workflow using GitHub Actions. It automates building and testing the Rust project across different toolchain versions (`stable`, `beta`, `nightly`).

## How to Run (Integration)
As `warp-prometheus` is a library, it is not "run" directly but rather integrated into a `warp` application.

To use this library in your `warp` service:
1.  Add `warp-prometheus` as a dependency in your `Cargo.toml`:
    ```toml
    [dependencies]
    warp-prometheus = "0.5.0" # Or the desired version
    prometheus = "0.13.0" # Ensure prometheus is also a direct dependency if you manage the registry
    warp = "0.3.0"
    ```
2.  Initialize a `prometheus::Registry` (or use the default one) and a `Vec<String>` specifying which path segments should retain their actual values (others will be replaced by `*`).
3.  Create an instance of `warp_prometheus::Metrics` with the registry and path includes.
4.  Apply the `metrics.http_metrics` function to your `warp` routes using `warp::log::custom`:

    ```rust
    use prometheus::Registry;
    use warp_prometheus::Metrics;
    use warp::Filter;

    // 1. Initialize Registry and specify path segments to include
    let registry: Registry = Registry::new();
    let path_includes: Vec<String> = vec![String::from("hello")]; // "hello" segment will be kept

    // 2. Define your warp routes
    let route_one = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });

    // 3. Create Metrics instance
    let metrics = Metrics::new(&registry, &path_includes);

    // 4. Apply metrics middleware to your routes
    let test_routes = route_one.with(warp::log::custom(move |log| {
        metrics.http_metrics(log)
    }));

    // In a full application, you would then serve these routes:
    // warp::serve(test_routes).run(([127, 0, 0, 1], 3030)).await;
    ```

## How to Test
The library includes unit tests for its core logic, specifically the path sanitization functionality.

To run the tests:
```bash
cargo test
```