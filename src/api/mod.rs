//! This module gives us all of the tools needed for interacting with the Overpass / OSM API
//! endpoint. This crate relies on [reqwest](https://docs.rs/reqwest/latest/reqwest/) to make this
//! possible but conviently hides the details of making HTTP requests to the endpoint.
//!
//! The API module provides to primary tools for the developer: the
//! [`crate::api::query_engine::QueryEngine`] and the
//! [`crate::api::overpass_response::OverpassResponse`].
//!
//! The query engine provides a simple interface for interacting with the API and the structure
//! provides some reasonable defaults of requests we might be interested in when we are using OSM
//! data for building graphs. Naturally, it's possible to modify these defaults to whatever your
//! end application demands.
//!
//! The overpass response exists because [serde_json](https://docs.rs/serde_json/latest/serde_json/)
//! can automatically parse json strings into structures, provided the structures have a shape that
//! matches the json. The response type from Overpass is very regular, so we can leverage this to
//! our advantage. If you are not using the Overpass API, you don't need this structure.

pub mod query_engine;
pub use query_engine::*;

pub mod overpass_response;
pub use overpass_response::*;
