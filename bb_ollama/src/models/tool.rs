use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: Function,
}

impl Tool {
    pub fn new() -> ToolBuilder {
        ToolBuilder::new()
    }
}

#[derive(Default)]
pub struct ToolBuilder {
    pub function_name: Option<String>,
    pub function_description: Option<String>,
    pub properties: HashMap<String, Property>,
    pub required_properties: Vec<String>,
}

impl ToolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn function_name(mut self, name: impl Into<String>) -> Self {
        self.function_name = Some(name.into());

        self
    }

    pub fn function_description(mut self, description: impl Into<String>) -> Self {
        self.function_description = Some(description.into());

        self
    }

    pub fn add_function_property(mut self, name: impl Into<String>, property: Property) -> Self {
        self.properties.insert(name.into(), property);

        self
    }

    pub fn add_required_property(mut self, property_name: impl Into<String>) -> Self {
        self.required_properties.push(property_name.into());

        self
    }

    pub fn build(self) -> Tool {
        let tool = Tool {
            tool_type: "function".to_owned(),
            function: Function {
                name: self
                    .function_name
                    .expect("Missing function name when building tool"),
                description: self
                    .function_description
                    .expect("Missing description when building tool"),
                parameters: Parameter {
                    parameter_type: "object".to_owned(),
                    properties: self.properties,
                    required: self.required_properties,
                },
            },
        };

        tool
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub property_type: PropertyType,
    pub description: String,
}

impl Property {
    pub fn new_string(description: impl Into<String>) -> Self {
        Self {
            property_type: PropertyType::String,
            description: description.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    String,
}
