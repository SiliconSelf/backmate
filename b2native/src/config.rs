//! Library configuration
//!
//! A few aspects of the library can be configured via environment variables prefixed with `B2NATIVE_`.

use std::sync::LazyLock;

use figment::{
    providers::Env,
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

/// A lazily-evaluated static containing the global library configuration
pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(Config::new);

/// A struct representing the library configuration
#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    /// The URL to call for the ``b2_authorize_account`` endpoint
    pub(crate) authorize_account_endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            authorize_account_endpoint:
                "https://api.backblazeb2.com/b2api/v3/b2_authorize_account"
                    .to_owned(),
        }
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("B2 Native API Library Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        figment::providers::Serialized::defaults(Self::default()).data()
    }
}

impl Config {
    /// Create a new instance of the library configuration
    pub(crate) fn new() -> Self {
        Figment::from(Self::default())
            .merge(Env::prefixed("B2NATIVE_"))
            .extract()
            .expect("Failed to configure B2 native library")
    }
}
