use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Resource {
    Command(ResourceData),
    Agent(ResourceData),
    Skill(ResourceData),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceData {
    pub name: String,
    pub plugin: String,
    pub content: String,
    pub metadata: Value,
}

impl Resource {
    pub fn name(&self) -> &str {
        match self {
            Resource::Command(d) | Resource::Agent(d) | Resource::Skill(d) => &d.name,
        }
    }

    pub fn plugin(&self) -> &str {
        match self {
            Resource::Command(d) | Resource::Agent(d) | Resource::Skill(d) => &d.plugin,
        }
    }
}
