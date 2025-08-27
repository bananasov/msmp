mod models;
mod utils;
mod generators;

pub use models::*;

use std::fs;
use std::path::Path;
use utils::split_methods_and_notifs;

use generators::{
    schema::generate_schemas,
    method::generate_methods,
    notification::generate_notifications,
};

pub struct GeneratedTypes {
    pub schemas: String,
    pub methods: String,
    pub notifications: String,
}

pub fn generate_types(data: &str) -> Result<GeneratedTypes, Box<dyn std::error::Error>> {
    let doc: OpenRpcDocument = serde_json::from_str(data)?;

    let (methods, notifications) = split_methods_and_notifs(&doc.result.methods);

    let schemas_code = generate_schemas(&doc.result.components.schemas);
    let methods_code = generate_methods(&methods, &doc.result.components.schemas);
    let notifications_code = generate_notifications(&notifications, &doc.result.components.schemas);

    Ok(GeneratedTypes {
        schemas: schemas_code,
        methods: methods_code,
        notifications: notifications_code,
    })
}

impl GeneratedTypes {
    pub fn write_to_files(
        &self,
        types_crate_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src_path = types_crate_path.join("src");

        fs::write(src_path.join("schemas.rs"), &self.schemas)?;
        fs::write(src_path.join("methods.rs"), &self.methods)?;
        fs::write(src_path.join("notifications.rs"), &self.notifications)?;

        println!("Generated types written to {}", src_path.display());
        Ok(())
    }
}

pub fn generate_and_write_types(
    data: &str,
    types_crate_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let types = generate_types(data)?;
    types.write_to_files(types_crate_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils::{escape_rust_keyword, safe_field_name};
    use super::*;

    static RPC_DISCOVER_RESPONSE: &'static str =
        include_str!("../../../data/rpc.discover-response.json");

    #[test]
    fn test_deserialize_openrpc_document() {
        let doc: OpenRpcDocument = serde_json::from_str(RPC_DISCOVER_RESPONSE).unwrap();

        assert_eq!(doc.result.openrpc, "1.3.2");
        assert_eq!(doc.result.info.title, "Minecraft Server JSON-RPC");
        assert_eq!(doc.result.info.version, "1.0.0");

        assert!(!doc.result.methods.is_empty());
        assert!(!doc.result.components.schemas.is_empty());

        println!(
            "Successfully deserialized OpenRPC document with {} methods and {} schemas",
            doc.result.methods.len(),
            doc.result.components.schemas.len()
        );
    }

    #[test]
    fn test_split_methods_and_notifications() {
        let doc: OpenRpcDocument = serde_json::from_str(RPC_DISCOVER_RESPONSE).unwrap();
        let (methods, notifications) = split_methods_and_notifs(&doc.result.methods);

        println!(
            "Found {} RPC methods and {} notifications",
            methods.len(),
            notifications.len()
        );

        for method in &methods {
            assert!(method.name.starts_with("minecraft:"));
        }

        for notification in &notifications {
            assert!(notification.name.starts_with("notification:"));
        }
    }

    #[test]
    fn test_generate_types() {
        let result = generate_types(RPC_DISCOVER_RESPONSE).unwrap();

        println!(
            "Generated schemas ({} bytes):\n{}",
            result.schemas.len(),
            &result.schemas[..result.schemas.len().min(500)]
        );
        println!(
            "Generated methods ({} bytes):\n{}",
            result.methods.len(),
            &result.methods[..result.methods.len().min(500)]
        );
        println!(
            "Generated notifications ({} bytes):\n{}",
            result.notifications.len(),
            &result.notifications[..result.notifications.len().min(500)]
        );

        assert!(!result.schemas.is_empty());
        assert!(!result.methods.is_empty());
        assert!(!result.notifications.is_empty());

        assert!(result.schemas.contains("pub struct"));
        assert!(result.methods.contains("pub struct"));
        assert!(result.notifications.contains("pub enum Notification"));
    }

    #[test]
    fn test_schema_reference_parsing() {
        let doc: OpenRpcDocument = serde_json::from_str(RPC_DISCOVER_RESPONSE).unwrap();

        let allowlist_method = doc
            .result
            .methods
            .iter()
            .find(|m| m.name == "minecraft:allowlist")
            .expect("Should find allowlist method");

        if let Some(result) = &allowlist_method.result {
            match &result.schema {
                SchemaReference::Inline(schema) => {
                    if let Some(items) = &schema.items {
                        match items.as_ref() {
                            SchemaReference::Reference { ref_path } => {
                                assert_eq!(ref_path, "#/components/schemas/player");
                            }
                            _ => panic!("Expected reference in array items"),
                        }
                    }
                }
                _ => panic!("Expected inline schema for result"),
            }
        }
    }

    #[test]
    fn test_rust_keyword_escaping() {
        assert_eq!(escape_rust_keyword("use"), "r#use");
        assert_eq!(escape_rust_keyword("type"), "r#type");
        assert_eq!(escape_rust_keyword("async"), "r#async");
        assert_eq!(escape_rust_keyword("normal_field"), "normal_field");

        assert_eq!(safe_field_name("use"), "r#use");
        assert_eq!(safe_field_name("normal_field"), "normal_field");
    }

    #[test]
    fn test_method_with_rust_keywords() {
        let json_with_keyword = r#"{
            "jsonrpc": "2.0",
            "result": {
                "openrpc": "1.3.2",
                "info": {"title": "Test", "version": "1.0.0"},
                "methods": [{
                    "name": "test:method",
                    "params": [{"name": "use", "schema": {"type": "boolean"}, "required": true}],
                    "result": {"name": "type", "schema": {"type": "string"}}
                }],
                "components": {"schemas": {}}
            }
        }"#;

        let result = generate_types(json_with_keyword).unwrap();

        assert!(result.methods.contains("r#use"));
        assert!(result.methods.contains("r#type"));
        assert!(result.methods.contains("#[serde(rename = \"use\")]"));
        assert!(result.methods.contains("#[serde(rename = \"type\")]"));
    }
}
