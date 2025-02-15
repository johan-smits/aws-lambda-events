use super::super::encodings::{Base64Data, MillisecondTimestamp};
use crate::custom_serde::*;
use std::collections::HashMap;

/// `KinesisFirehoseEvent` represents the input event from Amazon Kinesis Firehose. It is used as the input parameter.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisFirehoseEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invocation_id: Option<String>,
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub delivery_stream_arn: Option<String>,
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source_kinesis_stream_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub records: Vec<KinesisFirehoseEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisFirehoseEventRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub record_id: Option<String>,
    pub approximate_arrival_timestamp: MillisecondTimestamp,
    pub data: Base64Data,
    #[serde(rename = "kinesisRecordMetadata")]
    pub kinesis_firehose_record_metadata: KinesisFirehoseRecordMetadata,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisFirehoseResponse {
    pub records: Vec<KinesisFirehoseResponseRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisFirehoseResponseRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub record_id: Option<String>,
    /// The status of the transformation. May be TransformedStateOk, TransformedStateDropped or TransformedStateProcessingFailed
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub result: Option<String>,
    pub data: Base64Data,
    pub metadata: KinesisFirehoseResponseRecordMetadata,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisFirehoseResponseRecordMetadata {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub partition_keys: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisFirehoseRecordMetadata {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub shard_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub partition_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub sequence_number: Option<String>,
    pub subsequence_number: i64,
    pub approximate_arrival_timestamp: MillisecondTimestamp,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "firehose")]
    fn example_firehose_event() {
        let data = include_bytes!("fixtures/example-firehose-event.json");
        let parsed: KinesisFirehoseEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: KinesisFirehoseEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
