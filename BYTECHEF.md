# BYTECHEF.md

`warp-prometheus` is a Rust library designed to easily integrate Prometheus metrics into Warp web applications. It provides a `Metrics` struct that allows developers to collect HTTP request duration, method, status, and sanitized path information, publishing these metrics to a Prometheus registry.

## Architecture
The core of `warp-prometheus` is the `Metrics` struct, which encapsulates a `prometheus::HistogramVec` named `server_response_duration_seconds`. This histogram tracks HTTP response times, categorized by a `classifier` (method and sanitized path) and `status` code. The `Metrics::new` constructor initializes this histogram and registers it with a Prometheus `Registry`. The `sanitize_path_segments` helper method processes incoming request paths, replacing dynamic segments with a wildcard (`*`) unless they are explicitly specified in the `include_path_labels` list, ensuring metric cardinality is controlled. The main entry point for metric collection is `Metrics::http_metrics`, which is intended to be used as a custom log handler with `warp::log::custom`, automatically extracting relevant request information and observing the histogram.

## Key files
*   `Cargo.toml`: Defines the project as a Rust library named `warp-prometheus`, specifies its version, author, description, and key dependencies (`log`, `prometheus`, `warp`).
*   `src/lib.rs`: Contains the entire library's logic, including the `Metrics` struct, its methods (`new`, `sanitize_path_segments`, `http_metrics`), and internal unit tests. This file is central to how metrics are defined, collected, and exposed.
*   `README.md`: Serves as the primary introduction to the library, providing a high-level description, project badges, and a complete, runnable example demonstrating how to initialize and integrate `warp-prometheus` into a Warp application.

## How to run
This project is a library and does not have a standalone executable. To "run" it, integrate it into a Warp application by adding it as a dependency in your project's `Cargo.toml`:

```toml
[dependencies]
warp-prometheus = "0.5.0" # Or the latest version
```

Then, instantiate `Metrics` and apply `Metrics::http_metrics` to your Warp routes using `warp::log::custom`, as demonstrated in the `README.md` example.

## How to test
To execute the unit tests included in the library, navigate to the project root and run:

```bash
cargo test
```

The `.github/workflows/rust.yml` CI configuration also includes a `cargo test` step, ensuring tests are run on every push and pull request.