use super::Resource;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

#[derive(Default)]
pub struct Registry {
    /// Key: 리소스 이름 (파일명 기준)
    /// Value: Resource 객체
    resources: HashMap<String, Resource>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 리소스를 레지스트리에 등록합니다.
    /// 이름이 중복될 경우 에러를 반환합니다.
    pub fn register(&mut self, resource: Resource) -> Result<()> {
        let name = resource.name().to_string();

        if let Some(existing) = self.resources.get(&name) {
            return Err(anyhow!(
                "Conflict detected: Resource '{}' is defined in both '{}' and '{}'.",
                name,
                existing.plugin(),
                resource.plugin()
            ));
        }

        self.resources.insert(name, resource);
        Ok(())
    }

    pub fn all_resources(&self) -> Vec<&Resource> {
        self.resources.values().collect()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::resource::ResourceData;
    use serde_json::Value;

    fn mock_resource(name: &str, plugin: &str) -> Resource {
        Resource::Command(ResourceData {
            name: name.to_string(),
            plugin: plugin.to_string(),
            content: String::new(),
            metadata: Value::Null,
        })
    }

    #[test]
    fn test_register_and_conflict() {
        let mut registry = Registry::new();
        let res1 = mock_resource("foo", "plugin_a");
        let res2 = mock_resource("foo", "plugin_b");

        assert!(registry.register(res1).is_ok());
        let result = registry.register(res2);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Conflict detected"));
    }
}
