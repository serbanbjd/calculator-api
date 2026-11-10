//! Calculator API: a stateless HTTP calculator with exact decimal arithmetic.
//! See `docs/prd/PRD.md` for requirements and `docs/adr/` for design decisions.

pub mod config;
pub mod domain;
pub mod http;

pub use http::app;
