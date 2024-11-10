//! A collection of API endpoints and their structures

use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};
use reqwest::{Body, Request};
use serde::{Deserialize, Serialize};

pub(crate) mod b2_authorize_account;
mod b2_list_buckets;

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

enum ApiResult<R, E, F> {
    Response(ApiResponse<R, E>),
    Failure(F)
}

enum ApiResponse<R, E> {
    Ok(R),
    Error(E)
}

impl<R, E, F> FromResidual for ApiResult<R, E, F>
{
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::Failure(residual.unwrap_err())
    }
}

impl<R, E, F> Try for ApiResult<R, E, F> {
    type Output = ApiResponse<R, E>;
    type Residual = Result<Infallible, F>;

    fn from_output(output: Self::Output) -> Self {
        Self::Response(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            ApiResult::Response(r) => {ControlFlow::Continue(r)}
            ApiResult::Failure(f) => {ControlFlow::Break(Err(f))}
        }
    }
}

pub(crate) trait OutgoingRequest<'a, T>
where
    T: Serialize,
{
    type Response: Serialize + Deserialize<'a>;
    type Error;
    type Failure;
    async fn send(&mut self, body: T) -> ApiResult<Self::Response, Self::Error, Self::Failure>;
}
