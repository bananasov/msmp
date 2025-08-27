use convert_case::{Case, Casing};
use crate::models::{Method, Schema};
use crate::utils::{extract_notification_name, safe_field_name, schema_reference_to_rust_type};
use std::collections::HashMap;

pub fn generate_notifications(notifications: &[&Method], schemas: &HashMap<String, Schema>) -> String {
    let mut output = String::new();
    output.push_str("use serde::{Deserialize, Serialize};\n");
    output.push_str("use crate::schemas::*;\n\n");

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str("#[serde(tag = \"method\", content = \"params\")]\n");
    output.push_str("pub enum Notification {\n");

    for notification in notifications {
        let notif_name = extract_notification_name(&notification.name);
        let variant_name = notif_name.to_case(Case::Pascal);

        if let Some(description) = &notification.description {
            output.push_str(&format!("    /// {}\n", description));
        }

        output.push_str(&format!(
            "    #[serde(rename = \"{}\")]\n",
            notification.name
        ));

        if notification.params.is_empty() {
            output.push_str(&format!("    {},\n", variant_name));
        } else {
            output.push_str(&format!("    {}({}Params),\n", variant_name, variant_name));
        }
    }

    output.push_str("}\n\n");

    for notification in notifications {
        if !notification.params.is_empty() {
            let notif_name = extract_notification_name(&notification.name);
            let struct_name = format!("{}Params", notif_name.to_case(Case::Pascal));

            if let Some(description) = &notification.description {
                output.push_str(&format!(
                    "/// Parameters for {}\n",
                    description.to_lowercase()
                ));
            }

            output.push_str(&"#[derive(Debug, Clone, Serialize, Deserialize)]\n".to_string());
            output.push_str(&format!("pub struct {} {{\n", struct_name));

            for param in &notification.params {
                let field_name = safe_field_name(&param.name);
                let field_type = schema_reference_to_rust_type(&param.schema, schemas);

                if let Some(description) = &param.description {
                    output.push_str(&format!("    /// {}\n", description));
                }

                let original_snake_case = param.name.to_case(Case::Snake);
                if field_name != param.name || field_name != original_snake_case {
                    output.push_str(&format!("    #[serde(rename = \"{}\")]\n", param.name));
                }

                output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
            }

            output.push_str("}\n\n");
        }
    }

    output
}
