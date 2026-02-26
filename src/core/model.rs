use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::path::PathBuf;

/// 리소스 타입 정의
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Command,
    Agent,
    Skill,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ResourceType::Command => "command",
            ResourceType::Agent => "agent",
            ResourceType::Skill => "skill",
        };
        write!(f, "{}", s)
    }
}

/// 리소스 식별을 위한 키
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ResourceKey {
    pub plugin: String,
    pub r_type: String,
    pub name: String,
}

/// 리소스를 구성하는 파일 경로 그룹
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourcePaths {
    pub md: Option<PathBuf>,
    pub metadata: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceData {
    pub name: String,
    pub plugin: String,
    pub content: String,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Resource {
    Command(ResourceData),
    Agent(ResourceData),
    Skill(ResourceData),
}

impl Resource {
    pub fn r_type(&self) -> ResourceType {
        match self {
            Resource::Command(_) => ResourceType::Command,
            Resource::Agent(_) => ResourceType::Agent,
            Resource::Skill(_) => ResourceType::Skill,
        }
    }

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

/// 변환된 파일의 경로와 내용을 담는 구조체
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformedFile {
    /// 결과물이 저장될 상대 경로 (예: commands/foo.toml)
    pub path: PathBuf,
    /// 변환이 완료된 파일의 실제 내용
    pub content: String,
}
