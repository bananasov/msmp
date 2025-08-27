use convert_case::{Case, Casing};
use crate::models::{Method, Schema};
use crate::utils::{extract_method_name, safe_field_name, schema_reference_to_rust_type};
use std::collections::HashMap;

pub fn generate_methods(methods: &[&Method], schemas: &HashMap<String, Schema>) -> String {
    let mut output = String::new();
    output.push_str("use serde::{Deserialize, Serialize};\n");
    output.push_str("use crate::schemas::*;\n\n");

    for method in methods {
        let method_name = extract_method_name(&method.name);
        let struct_name = method_name.to_case(Case::Pascal);

        if let Some(description) = &method.description {
            output.push_str(&format!("/// {}\n", description));
            output.push_str(&format!("/// \n/// Method: `{}`\n", method.name));
        }

        if !method.params.is_empty() {
            output.push_str(&"#[derive(Debug, Clone, Serialize, Deserialize)]\n".to_string());
            output.push_str(&format!("pub struct {}Request {{\n", struct_name));

            for param in &method.params {
                let field_name = safe_field_name(&param.name);
                let field_type = schema_reference_to_rust_type(&param.schema, schemas);
                let is_required = param.required.unwrap_or(false);

                if let Some(description) = &param.description {
                    output.push_str(&format!("    /// {}\n", description));
                }

                let original_snake_case = param.name.to_case(Case::Snake);
                if field_name != param.name || field_name != original_snake_case {
                    output.push_str(&format!("    #[serde(rename = \"{}\")]\n", param.name));
                }

                if !is_required {
                    output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                }

                let final_type = if is_required {
                    field_type
                } else {
                    format!("Option<{}>", field_type)
                };

                output.push_str(&format!("    pub {}: {},\n", field_name, final_type));
            }

            output.push_str("}\n\n");
        }

        if let Some(result) = &method.result {
            let response_type = schema_reference_to_rust_type(&result.schema, schemas);
            output.push_str(&"#[derive(Debug, Clone, Serialize, Deserialize)]\n".to_string());
            output.push_str(&format!("pub struct {}Response {{\n", struct_name));

            if let Some(description) = &result.description {
                output.push_str(&format!("    /// {}\n", description));
            }

            let result_field_name = safe_field_name(&result.name);
            let original_snake_case = result.name.to_case(Case::Snake);
            if result_field_name != result.name || result_field_name != original_snake_case {
                output.push_str(&format!("    #[serde(rename = \"{}\")]\n", result.name));
            }

            output.push_str(&format!("    pub {}: {},\n", result_field_name, response_type));

            output.push_str("}\n\n");
        }
    }

    output
}
