# BYTECHEF.md

This document provides a concise overview of the `warp-prometheus` project, outlining its purpose, architecture, key components, and operational instructions.

## What the project does

This project provides a Rust library that integrates Prometheus metrics collection into Warp web applications. It offers a `Metrics` struct to track HTTP request durations and statuses, with configurable path sanitization for metric labels to ensure efficient and meaningful data collection.

## Architecture

The `warp-prometheus` library provides a `Metrics` struct that acts as an instrumentation layer for Warp applications. It registers a `HistogramVec` with a Prometheus `Registry` to record HTTP request durations, categorized by `classifier` (method + sanitized path) and `status` labels. The `http_metrics` method processes `warp::log::Info` objects, sanitizing request paths based on a configured list of `include_path_labels` to aggregate metrics efficiently, then observes the request duration in the histogram.

## Key files

*   `Cargo.toml`: Configures the Rust project, specifying package metadata, dependencies (`log`, `prometheus`, `warp`), and build settings.
*   `README.md`: Serves as the primary introduction, offering a project overview, badges, and a minimal code example demonstrating how to integrate `warp-prometheus`.
*   `src/lib.rs`: Contains the complete implementation of the `warp-prometheus` library. This includes:
    *   The `Metrics` struct for managing Prometheus metrics.
    *   The `new` constructor for initializing the `Metrics` instance and registering the histogram.
    *   The `sanitize_path_segments` utility for normalizing URL paths by replacing dynamic segments with wildcards, based on `include_path_labels`, to prevent high cardinality issues in Prometheus metrics.
    *   The `http_metrics` function, which is the main entry point for recording HTTP request data (duration, method, status, path) into Prometheus histograms.
*   `.github/workflows/rust.yml`: Defines the Continuous Integration (CI) pipeline using GitHub Actions, responsible for building and testing the Rust project on push and pull request events.

## How to run

This project is a Rust library. To use it, integrate it into your `Warp` application by adding `warp-prometheus` to your `Cargo.toml` dependencies.

*   **Build the library:**
    ```bash
    cargo build
    ```

## How to test

Run the unit tests for the library:

```bash
cargo test
```