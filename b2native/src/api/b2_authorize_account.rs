//! Functionality related to the ``b2_authorize_account`` endpoint
//!
//! [API Docs](https://www.backblaze.com/apidocs/b2-authorize-account)

use serde::{Deserialize, Serialize};
use crate::api::{ApiResult, OutgoingRequest};
use crate::{ApiError, Session};
use crate::config::CONFIG;

/// The request body
#[derive(Serialize, Deserialize)]
pub(crate) struct Request;

/// The expected response body
///
/// This response body structure correlates to the expected response structure
/// of a successful request.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response {
    /// The identifier for the account.
    pub(crate) account_id: String,
    /// A data structure that groups the information you need by API suite.
    pub(crate) api_info: ApiInfo,
    /// An authorization token to use with all calls, other than
    /// ``b2_authorize_account``, that need an Authorization header. This
    /// authorization token is valid for at most 24 hours.
    pub(crate) authorization_token: String,
    /// Expiration timestamp for the application key.
    pub(crate) application_key_expiration_timestamp: Option<usize>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiInfo {
    /// A data structure that contains the information you need for the Partner
    /// API.
    pub(crate) groups_api: Option<GroupsAPI>,
    /// A data structure that contains the information you need for the B2
    /// Native API.
    pub(crate) storage_api: StorageApi,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GroupsAPI {
    /// A list of strings, each one naming a capability the new key should
    /// have.
    pub(crate) capabilities: Vec<String>,
    /// The base URL for all Partner API calls.
    pub(crate) groups_api_url: String,
    /// The API type that the information in the object corresponds to.
    pub(crate) info_type: String,
}

/// A data structure that contains the information you need for the B2 Native
/// API.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StorageApi {
    /// The smallest possible size of a part of a large file (except the last
    /// one). This is smaller than the `recommendedPartSize`. If you use it,
    /// you may find that it takes longer overall to upload a large file.
    pub(crate) absolute_minimum_part_size: usize,
    /// The base URL to use for all API calls except for uploading and
    /// downloading files.
    pub(crate) api_url: String,
    /// When present, access is restricted to one bucket.
    pub(crate) bucket_id: Option<String>,
    ///When bucketId is set, and it is a valid bucket that has not been
    /// deleted, this field is set to the name of the bucket. It's possible
    /// that bucketId is set to a bucket that no longer exists, in which case
    /// this field will be null. It's also null when bucketId is null.
    pub(crate) bucket_name: Option<String>,
    /// A list of strings, each one naming a capability the key has.
    pub(crate) capabilities: Vec<Capability>,
    /// The base URL to use for downloading files.
    pub(crate) download_url: String,
    /// The API type that the information in the object corresponds to.
    pub(crate) info_type: String,
    /// When present, access is restricted to files whose names start with the
    /// prefix
    pub(crate) name_prefix: Option<String>,
    /// The recommended file part size.
    pub(crate) recommended_part_size: usize,
    /// The base URL to use for all API calls using the S3 compatible API.
    pub(crate) s3_api_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "camelCase")]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) enum Capability {
    DeleteFiles,
    DeleteKeys,
    ReadBucketEncryption,
    WriteKeys,
    WriteBuckets,
    WriteBucketNotifications,
    WriteBucketReplications,
    ReadBucketNotifications,
    ReadBucketReplications,
    DeleteBuckets,
    ReadBuckets,
    BypassGovernance,
    ReadFileLegalHolds,
    ReadFiles,
    ListAllBucketNames,
    ReadBucketRetentions,
    WriteBucketRetentions,
    WriteFileLegalHolds,
    ShareFiles,
    WriteFiles,
    ListKeys,
    ListBuckets,
    ListFiles,
    WriteFileRetentions,
    WriteBucketEncryption,
    ReadFileRetentions,
    #[serde(other)]
    Other,
}

#[cfg(test)]
mod tests {
    use crate::api::b2_authorize_account::Response;

    #[test]
    fn deserialize_ok() {
        assert!(serde_json::from_str::<Response>(
            r#"
{
  "accountId": "ACCOUNT_ID",
  "apiInfo": {
    "storageApi": {
      "absoluteMinimumPartSize": 5000000,
      "apiUrl": "https://api001.backblazeb2.com",
      "bucketId": null,
      "bucketName": null,
      "capabilities": [
        "deleteFiles",
        "deleteKeys",
        "readBucketEncryption",
        "writeKeys",
        "writeBuckets",
        "writeBucketReplications",
        "readBucketReplications",
        "deleteBuckets",
        "readBuckets",
        "bypassGovernance",
        "readFileLegalHolds",
        "readFiles",
        "listAllBucketNames",
        "readBucketNotifications",
        "readBucketRetentions",
        "writeBucketRetentions",
        "writeFileLegalHolds",
        "shareFiles",
        "writeFiles",
        "listKeys",
        "listBuckets",
        "listFiles",
        "writeFileRetentions",
        "writeBucketEncryption",
        "writeBucketNotifications",
        "readFileRetentions"
      ],
      "downloadUrl": "https://f001.backblazeb2.com",
      "infoType": "storageApi",
      "namePrefix": null,
      "recommendedPartSize": 100000000,
      "s3ApiUrl": "https://s3.us-west-001.backblazeb2.com"
    },
    "groupsApi": {
      "capabilities": [
        "all"
      ],
      "groupsApiUrl": "https://apiNNN.backblazeb2.com",
      "infoType": "groupsApi"
    }
  },
  "applicationKeyExpirationTimestamp": null,
  "authorizationToken": "AUTHORIZATION_TOKEN"
}"#,
        )
        .is_ok());
    }
}
