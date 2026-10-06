use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub fields: BTreeMap<String, Value>,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}
