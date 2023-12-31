//! HTTP Client for the middleware.

use std::{future::Future, io::Read};

use serde::{de::DeserializeOwned, Serialize};

/// HTTP REST Client for the middleware.
pub trait RestClient {
    /// Send a POST request to the given endpoint with the given body, and return the entire response.
    fn post_raw<R: Read>(
        &self,
        endpoint: &str,
        headers: &[(&str, &str)],
        body: R,
    ) -> impl Future<Output = Result<Vec<u8>, String>> + Send;

    /// Send a GET request to the given endpoint, and return the entire response.
    fn get_raw(
        &self,
        endpoint: &str,
        headers: &[(&str, &str)],
    ) -> impl Future<Output = Result<Vec<u8>, String>> + Send;
}

pub(crate) trait RestClientExt {
    fn post_json<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        headers: &[(&str, &str)],
        body: &T,
    ) -> impl Future<Output = Result<R, String>> + Send;

    fn get_json<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        headers: &[(&str, &str)],
    ) -> impl Future<Output = Result<R, String>> + Send;
}
