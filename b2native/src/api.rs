//! A collection of API endpoints and their structures

use serde::{Deserialize, Serialize};

pub(crate) mod b2_authorize_account;

/// A representation of an error returned from the Backblaze API
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ApiError {
    /// The numeric HTTP status code. Always matches the status in the HTTP
    /// response.
    pub status: usize,
    /// A single-identifier code that identifies the error.
    pub code: ApiErrorCode,
    /// A human-readable message, in English, saying what went wrong.
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    /// The requested bucket ID does not match an existing bucket.
    BadBucketId,
    /// The request had the wrong fields or illegal values. The message
    /// returned with the error will describe the problem.
    BadRequest,
    /// An undocumented error code returned by the Backblaze API
    #[serde(other)]
    Other,
}
