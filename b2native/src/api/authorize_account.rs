use serde::{Serialize, Deserialize};

pub(crate) struct Body;
pub(crate) enum Response {
    Ok(Box<OkResponse>),
    BadRequest(Box<BadRequestResponse>)
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OkResponse {
    /// The identifier for the account.
    account_id: String,
    /// A data structure that groups the information you need by API suite.
    api_info: ApiInfo,
    /// An authorization token to use with all calls, other than b2_authorize_account, that need an Authorization header. This authorization token is valid for at most 24 hours.
    authorization_token: String,
    /// Expiration timestamp for the application key.
    application_key_expiration_timestamp: Option<usize>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiInfo {
    /// A data structure that contains the information you need for the Partner API.
    groups_api: GroupsAPI,
    /// A data structure that contains the information you need for the B2 Native API.
    storage_api: StorageApi
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GroupsAPI {
    /// A list of strings, each one naming a capability the new key should have.
    capabilities: Vec<String>,
    /// The base URL for all Partner API calls.
    groups_api_url: String,
    /// The API type that the information in the object corresponds to.
    info_type: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StorageApi {
    /// The smallest possible size of a part of a large file (except the last one). This is smaller than the `recommendedPartSize`. If you use it, you may find that it takes longer overall to upload a large file.
    absolute_minimum_part_size: usize,
    /// The base URL to use for all API calls except for uploading and downloading files.
    api_url: String,
    /// When present,access is restricted to one bucket.
    bucket_id: Option<String>,
    ///When bucketId is set, and it is a valid bucket that has not been deleted, this field is set to the name of the bucket. It's possible that bucketId is set to a bucket that no longer exists, in which case this field will be null. It's also null when bucketId is null.
    bucket_name: Option<String>,
    /// A list of strings, each one naming a capability the key has.
    capabilities: Vec<Capabilities>,
    /// The base URL to use for downloading files.
    download_url: String,
    /// The API type that the information in the object corresponds to.
    info_type: String,
    /// When present, access is restricted to files whose names start with the prefix
    name_prefix: Option<String>,
    /// The recommended file part size.
    recommended_part_size: usize,
    /// The base URL to use for all API calls using the S3 compatible API.
    s3_api_url: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub(crate) enum Capabilities {
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
    Other
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BadRequestResponse {
    /// The numeric HTTP status code. Always matches the status in the HTTP response.
    status: usize,
    /// A single-identifier code that identifies the error.
    code: BadRequestCode,
    /// A human-readable message, in English, saying what went wrong.
    message: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BadRequestCode {
    /// The requested bucket ID does not match an existing bucket.
    BadBucketId,
    /// The request had the wrong fields or illegal values. The message returned with the error will describe the problem.
    BadRequest,
    #[serde(other)]
   Other
}