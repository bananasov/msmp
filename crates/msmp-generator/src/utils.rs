use convert_case::{Case, Casing};
use crate::models::{Method, SchemaReference, Schema};
use std::collections::HashMap;

pub fn split_methods_and_notifs(methods: &[Method]) -> (Vec<&Method>, Vec<&Method>) {
    let mut rpc_methods = Vec::new();
    let mut notifications = Vec::new();

    for method in methods {
        if method.name.starts_with("notification:") {
            notifications.push(method);
        } else {
            rpc_methods.push(method);
        }
    }

    (rpc_methods, notifications)
}

pub fn extract_method_name(full_name: &str) -> String {
    full_name
        .strip_prefix("minecraft:")
        .unwrap_or(full_name)
        .replace('/', "_")
        .replace(':', "_")
        .to_case(Case::Pascal)
}

pub fn extract_notification_name(full_name: &str) -> String {
    full_name
        .strip_prefix("notification:")
        .unwrap_or(full_name)
        .replace('/', "_")
        .replace(':', "_")
        .to_case(Case::Pascal)
}

pub fn escape_rust_keyword(name: &str) -> String {
    match name {
        // strict keywords (always reserved)
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" |
        "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" |
        "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" |
        "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" |

        // weak keywords (contextually reserved)
        "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" |
        "typeof" | "unsized" | "virtual" | "yield" |

        // reserved keywords for future use
        "async" | "await" | "dyn" | "try" => {
            format!("r#{}", name)
        }
        _ => name.to_string()
    }
}

pub fn safe_field_name(original_name: &str) -> String {
    let snake_case_name = original_name.to_case(Case::Snake);
    escape_rust_keyword(&snake_case_name)
}

pub fn get_schema_description(schema_ref: &SchemaReference) -> Option<&String> {
    match schema_ref {
        SchemaReference::Inline(schema) => schema.description.as_ref(),
        SchemaReference::Reference { .. } => None,
    }
}

pub fn schema_reference_to_rust_type(
    schema_ref: &SchemaReference,
    all_schemas: &HashMap<String, Schema>,
) -> String {
    match schema_ref {
        SchemaReference::Reference { ref_path } => {
            let schema_name = ref_path
                .strip_prefix("#/components/schemas/")
                .unwrap_or_else(|| ref_path.split('/').last().unwrap_or("Unknown"))
                .to_case(Case::Pascal);
            schema_name
        }
        SchemaReference::Inline(schema) => schema_to_rust_type(schema, all_schemas),
    }
}

pub fn schema_to_rust_type(schema: &Schema, all_schemas: &HashMap<String, Schema>) -> String {
    if schema.enum_values.is_some() {
        return "String".to_string();
    }

    match schema.schema_type.as_deref() {
        Some("string") => match schema.format.as_deref() {
            Some("date-time") => "chrono::DateTime<chrono::Utc>".to_string(),
            Some("date") => "chrono::NaiveDate".to_string(),
            Some("time") => "chrono::NaiveTime".to_string(),
            Some("uuid") => "uuid::Uuid".to_string(),
            Some("uri") => "url::Url".to_string(),
            _ => "String".to_string(),
        },
        Some("integer") => match (schema.minimum, schema.maximum) {
            (Some(min), Some(max)) if min >= 0.0 && max <= u32::MAX as f64 => "u32".to_string(),
            (Some(min), _) if min >= 0.0 => "u64".to_string(),
            _ => "i64".to_string(),
        },
        Some("number") => "f64".to_string(),
        Some("boolean") => "bool".to_string(),
        Some("array") => {
            if let Some(items) = &schema.items {
                let item_type = schema_reference_to_rust_type(items, all_schemas);
                format!("Vec<{}>", item_type)
            } else {
                "Vec<serde_json::Value>".to_string()
            }
        }
        Some("object") => {
            if schema.properties.is_some() {
                "std::collections::HashMap<String, serde_json::Value>".to_string()
            } else {
                "serde_json::Value".to_string()
            }
        }
        _ => "serde_json::Value".to_string(),
    }
}
