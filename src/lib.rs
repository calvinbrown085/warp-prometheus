use log::trace;
use prometheus::HistogramOpts;
use prometheus::{HistogramVec, Registry};

#[derive(Debug, Clone)]
pub struct Metrics {
    http_timer: HistogramVec,
    include_path_labels: Vec<String>,
}

impl Metrics {
    pub fn new(cr: &Registry, include_path_labels: &Vec<String>) -> Self {
        let internal_http_timer_opts = HistogramOpts::new(
            "server_response_duration_seconds",
            "Route response time in seconds.",
        );
        let internal_http_timer =
            HistogramVec::new(internal_http_timer_opts, &["classifier"]).unwrap();
        cr.register(Box::new(internal_http_timer.clone())).unwrap();

        Self {
            http_timer: internal_http_timer,
            include_path_labels: include_path_labels.clone(),
        }
    }

    fn sanitize_path_segments(&self, path: &str) -> String {
        let path_segments: Vec<&str> = path.split('/').collect();
        path_segments.iter().fold(String::new(), |acc, &path| {
            if self.include_path_labels.contains(&path.to_string()) {
                format!("{}/{}", acc, path)
            } else if path == "" {
                acc.to_string()
            } else {
                format!("{}/*", acc)
            }
        })
    }

    /// Get prometheus metrics per-route and how long each route takes.
    /// ```
    /// use prometheus::Registry;
    /// use warp::Filter;
    /// use warp_prometheus::Metrics;
    ///
    ///
    /// let registry: &Registry = prometheus::default_registry();
    /// let path_includes: Vec<String> = vec![String::from("hello")];
    ///
    /// let route_one = warp::path("hello")
    ///    .and(warp::path::param())
    ///    .and(warp::header("user-agent"))
    ///    .map(|param: String, agent: String| {
    ///        format!("Hello {}, whose agent is {}", param, agent)
    ///    });
    /// let metrics = Metrics::new(&registry, &path_includes);
    ///
    /// let test_routes = route_one.with(warp::log::custom(move |log| {
    ///            metrics.http_metrics(log)
    ///        }));
    ///
    /// ```
    pub fn http_metrics(&self, info: warp::log::Info) {
        trace!(
            "Metric Status: {} - Method: {} - Path: {}",
            &info.status().as_u16().to_string(),
            &info.method(),
            &info.path()
        );
        let sanitized_classifier = format!(
            "{} - {}",
            info.method(),
            self.sanitize_path_segments(info.path())
        );
        self.http_timer
            .with_label_values(&[&sanitized_classifier])
            .observe(info.elapsed().as_secs_f64());

    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sanitize_path() {
        let registry: Registry = Registry::new();
        let path_includes: Vec<String> = vec![String::from("users"), String::from("registration")];

        let metrics = Metrics::new(&registry, &path_includes);
        let path = "/users/12345/registration/9797731279";
        let sanitized_path = metrics.sanitize_path_segments(path);

        assert_eq!("/users/*/registration/*".to_string(), sanitized_path)
    }

    #[test]
    fn test_sanitize_path_with_value_first() {
        let registry: Registry = Registry::new();
        let path_includes: Vec<String> = vec![String::from("users"), String::from("registration")];

        let metrics = Metrics::new(&registry, &path_includes);
        let path = "12344235/users/12345/12314151252/registration";
        let sanitized_path = metrics.sanitize_path_segments(path);

        assert_eq!("/*/users/*/*/registration".to_string(), sanitized_path)
    }

    #[test]
    fn test_sanitize_path_with_multiple_segments_in_order() {
        let registry: Registry = Registry::new();
        let path_includes: Vec<String> = vec![String::from("users"), String::from("registration")];

        let metrics = Metrics::new(&registry, &path_includes);
        let path = "/users/12345/12314151252/registration";
        let sanitized_path = metrics.sanitize_path_segments(path);

        assert_eq!("/users/*/*/registration".to_string(), sanitized_path)
    }

    #[test]
    fn test_totally_wrong_path() {

        let registry: Registry = Registry::new();
        let path_includes: Vec<String> = vec![String::from("users"), String::from("registration")];

        let metrics = Metrics::new(&registry, &path_includes);
        let path = "12344235/12141242/12345/12314151252/235235235";
        let sanitized_path = metrics.sanitize_path_segments(path);

        assert_eq!("/*/*/*/*/*".to_string(), sanitized_path)
    }
}
