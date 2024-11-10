mod api;

pub struct Session {
    token: String
}

impl Session {
    /// Create a new session
    ///
    /// This function calls the [`b2_authorize_account`](https://www.backblaze.com/apidocs/b2-authorize-account) endpoint to retrieve an authorization token and establish a new session
    pub fn new<S: Into<String>>(application_key_id: S, application_key: S) -> Self {
        todo!();
    }
}