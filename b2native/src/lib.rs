#![doc = include_str!("../README.md")]

mod api;
mod config;

pub use api::ApiError;
use reqwest::{Client, Error};

use crate::config::CONFIG;

/// A session for interacting with the Backblaze API
#[derive(Debug)]
pub struct Session {
    /// The authorization token this session will use
    _token: String,
    /// The HTTP client this session will reuse to take advantage of connection
    /// pooling
    _http_client: Client,
}

/// Errors that can be returned in the creation or use of a ``Session``
#[derive(Debug)]
pub enum SessionError {
    /// The library failed to send the request to the Backblaze API server.
    ///
    /// This is usually do to connectivity reasons.
    RequestFailed,
    /// Authentication was rejected by the remote server.
    ///
    /// This is usually because the user gave incorrect inputs.
    AuthenticationRejected {
        /// The machine-readable error code returned with the rejection
        code: api::ApiErrorCode,
        /// The human-readable error message returned with the rejection
        message: String,
    },
    /// The Backblaze API server returned a 200 status code, but
    /// deserialization failed
    ///
    /// This should only happen if the version of this library you're using
    /// doesn't line up with the API version served by Backblaze.
    SuccessfulDeserializationFailed,
    /// The Backblaze API server returned an error status code, but
    /// deserialization failed.
    ///
    /// This should only happen if the version of this library you're using
    /// doesn't line up with the API version served by Backblaze.
    ErrorDeserializationFailed,
}

impl From<Error> for SessionError {
    fn from(_value: Error) -> Self {
        Self::RequestFailed
    }
}

impl Session {
    /// Create a new session
    ///
    /// This function calls the [`b2_authorize_account`](https://www.backblaze.com/apidocs/b2-authorize-account) endpoint to retrieve an authorization token and establish a new session
    ///
    /// # Errors
    ///
    /// This function can return the following errors:
    /// - `SessionError:RequestFailed`
    /// - `SessionError::SuccessfulDeserializationFailed`
    /// - `SessionError::ErrorDeserializationFailed`
    pub async fn try_new<S: Into<String>>(
        application_key_id: S,
        application_key: S,
    ) -> Result<Self, SessionError> {
        let client = Client::new();
        let response = client
            .get(&CONFIG.authorize_account_endpoint)
            .basic_auth(application_key_id.into(), Some(application_key.into()))
            .send()
            .await?;
        if response.status().is_success() {
            if let Ok(body) =
                response.json::<api::b2_authorize_account::Response>().await
            {
                Ok(Self {
                    _token: body.authorization_token,
                    _http_client: client,
                })
            } else {
                Err(SessionError::SuccessfulDeserializationFailed)
            }
        } else if let Ok(error) = response.json::<ApiError>().await {
            Err(SessionError::AuthenticationRejected {
                code: error.code,
                message: error.message,
            })
        } else {
            Err(SessionError::ErrorDeserializationFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[cfg(feature = "integration-tests")]
    async fn test_auth() {
        let session = crate::Session::try_new(
            include_str!("../../key_id"),
            include_str!("../../key"),
        )
        .await;
        assert!(session.is_ok());
    }
}
