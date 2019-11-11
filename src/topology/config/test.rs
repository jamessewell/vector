use crate::conditions::ConditionConfig;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TestDefinition {
    name: String,
    input: TestInput,
    outputs: Vec<TestOutput>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TestInput {
    insert_at: String,
    #[serde(default = "default_input_type", rename = "type")]
    type_str: String,
    value: Option<String>,
}

fn default_input_type() -> String {
    "raw".to_string()
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TestOutput {
    extract_from: String,
    conditions: IndexMap<String, TestCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum TestCondition {
    String(String),
    Embedded(Box<dyn ConditionConfig>),
}