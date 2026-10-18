//! `http_rfc` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl`.
pub const HTTP_RFC_HNL: &str = include_str!("index.hnl");
