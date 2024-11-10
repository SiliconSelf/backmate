use std::sync::LazyLock;

use figment::{
    providers::Env,
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new());

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
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
    pub(crate) fn new() -> Self {
        Figment::from(Self::default())
            .merge(Env::prefixed("B2NATIVE_"))
            .extract()
            .expect("Failed to configure B2 native library")
    }
}
