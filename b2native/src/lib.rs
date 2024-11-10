mod api;
mod config;

use crate::config::CONFIG;
use reqwest::{Client, Error};

#[derive(Debug)]
pub struct Session {
    _token: String,
    _http_client: Client,
}

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
        code: api::ApiErrorCode,
        message: String
    },
    /// The Backblaze API server returned a 200 status code, but deserialization failed
    ///
    /// This should only happen if the version of this library you're using doesn't line up with the API version served by Backblaze.
    SuccessfulDeserializationFailed,
    /// The Backblaze API server returned an error status code, but deserialization failed.
    ///
    /// This should only happen if the version of this library you're using doesn't line up with the API version served by Backblaze.
    ErrorDeserializationFailed
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
            if let Ok(body) = response.json::<api::b2_authorize_account::Response>()
                .await {
                Ok(Self {
                    _token: body.authorization_token,
                    _http_client: client
                })
            } else {
                Err(SessionError::SuccessfulDeserializationFailed)
            }
        } else if let Ok(error) = response.json::<api::ApiError>().await {
            Err(SessionError::AuthenticationRejected {
                code: error.code,
                message: error.message
            })
        } else {
            Err(SessionError::ErrorDeserializationFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Session;

    #[tokio::test]
    async fn test_auth() {
        let session = Session::try_new(
            include_str!("../../key_id"),
            include_str!("../../key")
        ).await;
        assert!(session.is_ok())
    }
}