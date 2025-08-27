use convert_case::{Case, Casing};
use crate::models::Schema;
use crate::utils::{safe_field_name, get_schema_description, schema_reference_to_rust_type};
use std::collections::HashMap;

pub fn generate_schemas(schemas: &HashMap<String, Schema>) -> String {
    let mut output = String::new();
    output.push_str("use serde::{Deserialize, Serialize};\n\n");

    let mut sorted_schemas: Vec<_> = schemas.iter().collect();
    sorted_schemas.sort_by_key(|(name, _)| *name);

    for (name, schema) in sorted_schemas {
        let struct_name = name.to_case(Case::Pascal);

        if let Some(description) = &schema.description {
            output.push_str(&format!("/// {}\n", description));
        }

        output.push_str(&generate_schema_struct(&struct_name, schema, schemas));
        output.push_str("\n\n");
    }

    output
}

fn generate_schema_struct(
    name: &str,
    schema: &Schema,
    all_schemas: &HashMap<String, Schema>,
) -> String {
    let mut output = String::new();

    if let Some(enum_values) = &schema.enum_values {
        return generate_enum_type(name, enum_values);
    }

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");

    if schema.additional_properties == Some(false) {
        output.push_str("#[serde(deny_unknown_fields)]\n");
    }

    output.push_str(&format!("pub struct {} {{\n", name));

    if let Some(properties) = &schema.properties {
        let required_fields: std::collections::HashSet<_> = schema
            .required
            .as_ref()
            .map(|r| r.iter().collect())
            .unwrap_or_default();

        let mut sorted_props: Vec<_> = properties.iter().collect();
        sorted_props.sort_by_key(|(name, _)| *name);

        for (prop_name, prop_schema) in sorted_props {
            let field_name = safe_field_name(prop_name);
            let field_type = schema_reference_to_rust_type(prop_schema, all_schemas);
            let is_required = required_fields.contains(prop_name);

            if let Some(description) = get_schema_description(prop_schema) {
                output.push_str(&format!("    /// {}\n", description));
            }

            let original_snake_case = prop_name.to_case(Case::Snake);
            if field_name != *prop_name || field_name != original_snake_case {
                output.push_str(&format!("    #[serde(rename = \"{}\")]\n", prop_name));
            }

            let final_type = if is_required {
                field_type
            } else {
                output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                format!("Option<{}>", field_type)
            };

            output.push_str(&format!("    pub {}: {},\n", field_name, final_type));
        }
    }

    output.push_str("}");
    output
}

fn generate_enum_type(name: &str, values: &[String]) -> String {
    let mut output = String::new();

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");

    let needs_string_serialization = values.iter().any(|v| v.to_case(Case::Pascal) != *v);

    if needs_string_serialization {
        output.push_str("#[serde(rename_all = \"snake_case\")]\n");
    }

    output.push_str(&format!("pub enum {} {{\n", name));

    for value in values {
        let variant_name = if value.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
            value.to_case(Case::Pascal)
        } else {
            value.to_case(Case::Pascal)
        };

        if needs_string_serialization && variant_name.to_lowercase() != value.to_lowercase() {
            output.push_str(&format!("    #[serde(rename = \"{}\")]\n", value));
        }

        output.push_str(&format!("    {},\n", variant_name));
    }

    output.push_str("}");
    output
}
