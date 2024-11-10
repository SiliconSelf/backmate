//! Functionality related to the ``b2_list_buckets`` endpoint
//!
//! [API Docs](https://www.backblaze.com/apidocs/b2-list-buckets)

use serde::{Deserialize, Serialize};

use crate::{api::OutgoingRequest, ApiError, Session};
use crate::api::ApiResult;

/// The request body
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request {
    /// Your account ID.
    account_id: String,
    /// When bucketId is specified, the result will be a list containing just
    /// this bucket, if it's present in the account, or no buckets if the
    /// account does not have a bucket with this ID.
    bucket_id: Option<String>,
    /// When bucketName is specified, the result will be a list containing just
    /// this bucket, if it's present in the account, or no buckets if the
    /// account does not have a bucket with this name.
    bucket_name: Option<String>,
    /// If present, this will be used as a filter for bucket types returned in
    /// the list buckets response. If not present, only buckets with bucket
    /// types "allPublic", "allPrivate" and "snapshot" will be returned. A
    /// special filter value of ["all"] will return all bucket types.
    ///
    /// If present, it must be in the form of a json array of strings
    /// containing valid bucket types in quotes and separated by a comma. Valid
    /// bucket types include "allPrivate", "allPublic", "restricted",
    /// "snapshot", "shared", and other values added in the future.
    ///
    /// A bad request error will be returned if "all" is used with other
    /// bucketTypes, bucketTypes is empty, or invalid bucketTypes are
    /// requested.
    bucket_types: Option<Vec<BucketType>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum BucketType {
    All,
    AllPrivate,
    AllPublic,
    Restricted,
    Snapshot,
    Shared,
    #[serde(other)]
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response {
    /// An array of bucket objects.
    pub(crate) buckets: Vec<Bucket>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Bucket {
    /// The account that the bucket is in.
    account_id: String,
    /// The unique identifier of the bucket.
    bucket_id: String,
    /// The unique name of the bucket
    bucket_name: String,
    /// One of: allPublic, allPrivate, restricted, snapshot, shared, or other
    /// values added in the future. allPublic means that anybody can download
    /// the files is the bucket; allPrivate means that you need an
    /// authorization token to download them; snapshot means that it's a
    /// private bucket containing snapshots created in the Backblaze web UI.
    bucket_type: BucketType,
    /// The user data stored with this bucket.
    ///
    /// Backblaze documentation does not give a useful example (`{}`) of the
    /// structure of this value, so it is defined as a `serde_json::Value`.
    bucket_info: serde_json::Value,
    /// The initial list (a JSON array) of CORS rules for this bucket.
    ///
    /// See [CORS Rules](https://www.backblaze.com/docs/cloud-storage-cross-origin-resource-sharing-rules) for an overview and the rule structure.
    cors_rules: serde_json::Value,
    /// The Object Lock configuration for this bucket.
    ///
    /// This field is filtered
    /// based on application key capabilities; readBucketRetentions capability
    /// is required to access the value. See Object Lock for more details on
    /// response structure.
    file_lock_configuration: FileLockConfiguration,
    /// The default bucket Server-Side Encryption settings for new files
    /// uploaded to this bucket.
    ///
    /// This field is filtered based on application
    /// key capabilities; readBucketEncryption capability is required to access
    /// the value. See Server-Side Encryption for more details on response
    /// structure.
    default_server_side_encryption: ServerSideEncryption,
    /// The list of lifecycle rules for this bucket.
    ///
    /// See [Lifecycle Rules](https://www.backblaze.com/docs/cloud-storage-lifecycle-rules) for an overview and the rule structure.
    lifecycle_rules: LifecycleRules,
    /// The list of replication rules for this bucket.
    ///
    /// See [Cloud Replication Rules](https://www.backblaze.com/docs/cloud-storage-create-a-cloud-replication-rule-with-the-native-api#file-name-prefixes) for an overview and the rule structure.
    replication_configuration: serde_json::Value,
    revision: usize,
    options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileLockConfiguration {
    is_client_authorized_to_read: bool,
    value: FileLockConfigurationValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileLockConfigurationValue {
    is_file_lock_enabled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileLockConfigurationValueRetention {
    mode: String,
    period: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServerSideEncryption {
    is_client_authorized_to_read: bool,
    value: ServerSideEncryptionValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServerSideEncryptionValue {
    algorithm: String,
    mode: String,
}

/// Lifecycle Rules
///
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LifecycleRules {
    /// Files are automatically hidden when they are replaced with a new
    /// version of the file, or they can be explicitly hidden using
    /// b2_hide_file. The daysFromHidingToDeleting property causes hidden files
    /// that you specify to be automatically deleted after a number of days.
    ///
    /// Because files are automatically hidden when replaced by a newer
    /// version, if you set this property to 10, then the older version is
    /// deleted 10 days after you upload a newer version of the file.
    days_from_hiding_to_deleting: usize,
    /// This value causes the specified files to be automatically hidden after
    /// a designated number of days.
    ///
    /// This applies to all of the copies of the file, even the most current
    /// version. For example, setting this property to 30 hides all of the
    /// files in the specified folder 30 days after you upload the files. This
    /// includes the most current version of the file. Use this option only if
    /// you want to hide all of the versions of the specified files. Valid
    /// values are null or numbers one and greater. Null means that no files
    /// are hidden based on this rule.
    days_from_uploading_to_hiding: usize,
    /// This setting cancels any unfinished large file versions after a given
    /// number of days.
    ///
    /// This can be useful for an automatic clean-up of any interrupted large
    /// file uploads. Using this Lifecycle Rule is similar to calling
    /// b2_cancel_large_file on the unfinished large files. During Lifecycle
    /// Rule processing, unfinished large files cannot hide other files, or be
    /// hidden by other files.
    days_from_starting_to_canceling_unfinished_large_files: usize,
    /// This property specifies the files in the bucket to which the Lifecycle
    /// Rule applies.
    ///
    /// For example, a value of working/ applies to all of the files in the
    /// folder named "working." Any file name that begins with this prefix is
    /// subject to the rule. A prefix of an empty string ("") means that the
    /// rule applies to all of the files in the bucket. When you set a custom
    /// rule through the Web UI that applies to all files in a bucket, the
    /// fileNamePrefix must be left blank.
    file_name_prefix: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplicationConfiguration {
    as_replication_source: ReplicationSource,
    as_replication_destination: ReplicationDestination,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplicationSource {
    replication_rules: ReplicationRules,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplicationRules {
    destination_bucket_id: String,
    file_name_prefix: String,
    include_existing_files: bool,
    is_enabled: bool,
    priority: usize,
    replication_rule_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplicationDestination {
    source_to_destination_key_mapping: String,
}

impl OutgoingRequest<'_, Request> for Session {
    type Response = Response;
    type Error = ApiError;
    type Failure = reqwest::Error;

    async fn send(&mut self, body: Request) -> ApiResult<Self::Response, Self::Error, Self::Failure> {
        let url = format!(
            "{}/b2api/v3/b2_list_buckets",
            self.storage_api_info.api_url
        );
        let body_ser = serde_json::to_string(&body)
            .expect("Failed to serialize request body");
        let mut request = self.http_client.post(&url)
            .header("Authorization", &self.token)
            .body(body_ser)
            .build()?;
        let response = self.http_client
            .execute(request)
            .await?;
        todo!();
    }
}
