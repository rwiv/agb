use crate::core::{Resource, ResourceType};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

#[derive(Default)]
pub struct Registry {
    /// Key: (ResourceType, Name)
    /// Value: Resource 객체
    resources: HashMap<(ResourceType, String), Resource>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 리소스를 레지스트리에 등록합니다.
    /// 동일한 타입과 이름을 가진 리소스가 이미 존재할 경우 에러를 반환합니다.
    pub fn register(&mut self, resource: Resource) -> Result<()> {
        let r_type = resource.r_type();
        let name = resource.name().to_string();
        let key = (r_type, name.clone());

        if let Some(existing) = self.resources.get(&key) {
            return Err(anyhow!(
                "Conflict detected: {} '{}' is defined in both '{}' and '{}'.",
                r_type,
                name,
                existing.plugin(),
                resource.plugin()
            ));
        }

        self.resources.insert(key, resource);
        Ok(())
    }

    pub fn all_resources(&self) -> Vec<&Resource> {
        self.resources.values().collect()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ResourceData, SkillData};
    use serde_json::Value;
    use std::path::PathBuf;

    fn mock_resource(name: &str, plugin: &str, r_type: ResourceType) -> Resource {
        let data = ResourceData {
            name: name.to_string(),
            plugin: plugin.to_string(),
            content: String::new(),
            metadata: Value::Null,
            source_path: PathBuf::from("mock/path"),
        };
        match r_type {
            ResourceType::Command => Resource::Command(data),
            ResourceType::Agent => Resource::Agent(data),
            ResourceType::Skill => Resource::Skill(SkillData {
                base: data,
                extras: Vec::new(),
            }),
        }
    }

    #[test]
    fn test_register_and_conflict() {
        let mut registry = Registry::new();
        let res1 = mock_resource("foo", "plugin_a", ResourceType::Command);
        let res2 = mock_resource("foo", "plugin_b", ResourceType::Command);

        assert!(registry.register(res1).is_ok());
        let result = registry.register(res2);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Conflict detected: command 'foo'")
        );
    }

    #[test]
    fn test_register_same_name_different_type() {
        let mut registry = Registry::new();
        let res1 = mock_resource("write-plan", "plugin_a", ResourceType::Command);
        let res2 = mock_resource("write-plan", "plugin_b", ResourceType::Skill);

        assert!(registry.register(res1).is_ok());
        assert!(registry.register(res2).is_ok());
        assert_eq!(registry.len(), 2);
    }
}
