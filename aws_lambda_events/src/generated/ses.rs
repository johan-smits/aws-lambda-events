use crate::custom_serde::*;
use chrono::{DateTime, Utc};

/// `SimpleEmailEvent` is the outer structure of an event sent via SES.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SimpleEmailRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub event_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub event_source: Option<String>,
    pub ses: SimpleEmailService,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailService {
    pub mail: SimpleEmailMessage,
    pub receipt: SimpleEmailReceipt,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailMessage {
    pub common_headers: SimpleEmailCommonHeaders,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub destination: Vec<String>,
    pub headers: Vec<SimpleEmailHeader>,
    pub headers_truncated: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailReceipt {
    pub recipients: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub spam_verdict: SimpleEmailVerdict,
    pub dkim_verdict: SimpleEmailVerdict,
    pub dmarc_verdict: SimpleEmailVerdict,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub dmarc_policy: Option<String>,
    pub spf_verdict: SimpleEmailVerdict,
    pub virus_verdict: SimpleEmailVerdict,
    pub action: SimpleEmailReceiptAction,
    pub processing_time_millis: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailHeader {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailCommonHeaders {
    pub from: Vec<String>,
    pub to: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub return_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub date: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub subject: Option<String>,
}

/// `SimpleEmailReceiptAction` is a logical union of fields present in all action
/// Types. For example, the FunctionARN and InvocationType fields are only
/// present for the Lambda Type, and the BucketName and ObjectKey fields are only
/// present for the S3 Type.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailReceiptAction {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub type_: Option<String>,
    pub topic_arn: Option<String>,
    pub bucket_name: Option<String>,
    pub object_key: Option<String>,
    pub smtp_reply_code: Option<String>,
    pub status_code: Option<String>,
    pub message: Option<String>,
    pub sender: Option<String>,
    pub invocation_type: Option<String>,
    pub function_arn: Option<String>,
    pub organization_arn: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailVerdict {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub status: Option<String>,
}

pub type SimpleEmailDispositionValue = String;

/// `SimpleEmailDisposition` disposition return for SES to control rule functions
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleEmailDisposition {
    pub disposition: SimpleEmailDispositionValue,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "ses")]
    fn example_ses_lambda_event() {
        let data = include_bytes!("fixtures/example-ses-lambda-event.json");
        let parsed: SimpleEmailEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SimpleEmailEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "ses")]
    fn example_ses_s3_event() {
        let data = include_bytes!("fixtures/example-ses-s3-event.json");
        let parsed: SimpleEmailEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SimpleEmailEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "ses")]
    fn example_ses_sns_event() {
        let data = include_bytes!("fixtures/example-ses-sns-event.json");
        let parsed: SimpleEmailEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SimpleEmailEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
