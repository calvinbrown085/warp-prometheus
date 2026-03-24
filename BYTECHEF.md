# BYTECHEF.md

## Project Description

`warp-prometheus` is a Rust library designed to provide "afterthought" Prometheus metrics for applications built with the Warp web framework. It simplifies the collection of HTTP request duration and status code metrics, allowing developers to instrument their Warp services with Observability features easily. The library includes functionality to sanitize URL paths, preventing high-cardinality issues with Prometheus labels.

## Architecture

The core of `warp-prometheus` is the `Metrics` struct. Upon instantiation with a Prometheus `Registry` and a vector of path segments to include, it registers a `server_response_duration_seconds` `HistogramVec`. This histogram tracks HTTP request durations, categorized by `classifier` (HTTP method combined with a sanitized path) and `status` (HTTP status code). The `Metrics` struct offers an `http_metrics` method, intended to be integrated into Warp route chains via `warp::log::custom`. This method processes `warp::log::Info` to extract relevant request details and records them to the registered histogram. A crucial `sanitize_path_segments` helper method automatically replaces dynamic path parts with wildcards (`*`) unless explicitly whitelisted, ensuring metric labels remain manageable.

## Key Files

*   `Cargo.toml`: Defines the project as a Rust library, specifies its `warp-prometheus` name, version, author, description, and declares essential dependencies including `log`, `prometheus`, and `warp`.
*   `README.md`: Serves as the primary introduction to the library, providing a high-level overview, links to documentation and CI status, and a practical code example demonstrating basic usage with a Warp filter.
*   `src/lib.rs`: Contains the full implementation of the `Metrics` struct. This file defines the `new` constructor, the `sanitize_path_segments` logic for controlling metric cardinality, and the `http_metrics` function that hooks into Warp's logging mechanism to record Prometheus metrics. It also includes unit tests for path sanitization.
*   `.github/workflows/rust.yml`: Configures the Continuous Integration (CI) pipeline for the project using GitHub Actions, ensuring that the Rust code builds and passes tests on every push and pull request.

## How to Run

This project is a library and is not run directly as an executable. To use `warp-prometheus` in your Warp application:

1.  **Add to Dependencies:** Include `warp-prometheus` in your project's `Cargo.toml`:
    ```toml
    [dependencies]
    warp-prometheus = "0.5.0" # Use the appropriate version
    # ... other dependencies like warp, prometheus
    ```
2.  **Integrate Metrics:** In your Warp application code, instantiate `Metrics` with a Prometheus `Registry` and a list of desired path segments to include in metric labels. Then, apply the `http_metrics` function to your Warp routes:
    ```rust
    use prometheus::Registry;
    use warp_prometheus::Metrics;
    use warp::Filter;

    let registry: Registry = Registry::new();
    let path_includes: Vec<String> = vec![String::from("hello")]; // Paths to include as specific labels

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

    // You would then serve test_routes as part of your warp application
    // warp::serve(test_routes).run(([127, 0, 0, 1], 3030)).await;
    ```

## How to Test

To execute the unit tests for the `warp-prometheus` library, use the standard Cargo test command:

```bash
cargo test
```

This will run all tests defined in `src/lib.rs`, primarily verifying the functionality of the `sanitize_path_segments` method.