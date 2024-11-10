use serde::{Serialize, Deserialize};

pub(crate) mod b2_authorize_account;

pub(crate) enum Error {
    /// The provided value could not be deserialized
    DeserializationFailed
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ApiError {
    pub status: usize,
    pub code: ApiErrorCode,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    /// The requested bucket ID does not match an existing bucket.
    BadBucketId,
    /// The request had the wrong fields or illegal values. The message
    /// returned with the error will describe the problem.
    BadRequest,
    #[serde(other)]
    Other,
}