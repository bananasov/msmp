use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OpenRpcDocument {
    pub jsonrpc: Option<String>,
    pub id: Option<serde_json::Value>,
    pub result: OpenRpcResult,
}

#[derive(Debug, Deserialize)]
pub struct OpenRpcResult {
    pub openrpc: String,
    pub info: Info,
    pub methods: Vec<Method>,
    pub components: Components,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    pub name: String,
    pub description: Option<String>,
    pub params: Vec<Parameter>,
    pub result: Option<ResultSchema>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub schema: SchemaReference,
    pub required: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResultSchema {
    pub name: String,
    pub schema: SchemaReference,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SchemaReference {
    Reference {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
    Inline(Schema),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub properties: Option<HashMap<String, SchemaReference>>,
    pub items: Option<Box<SchemaReference>>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    pub description: Option<String>,
    pub required: Option<Vec<String>>,
    pub title: Option<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<bool>,
    pub format: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<u32>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u32>,
    pub pattern: Option<String>,
}
