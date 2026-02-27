use crate::core::{BuildTarget, MetadataMap};
use anyhow::Result;
use serde_json::{Value, json};

pub struct MetadataMerger<'a> {
    target: BuildTarget,
    map: Option<&'a MetadataMap>,
}

impl<'a> MetadataMerger<'a> {
    pub fn new(target: BuildTarget, map: Option<&'a MetadataMap>) -> Self {
        Self { target, map }
    }

    /// Frontmatter(base)와 외부 YAML(external)을 병합합니다.
    pub fn merge(&self, base: &Value, external: Option<&Value>) -> Result<Value> {
        let mut merged = base.clone();

        // 1. Metadata Map 적용
        if let Some(map) = self.map {
            self.apply_mapping(&mut merged, map)?;
        }

        // 2. 외부 YAML 병합
        if let Some(ext) = external {
            self.apply_external_override(&mut merged, ext)?;
        }

        // 3. 예약어 정리
        self.cleanup(&mut merged);

        Ok(merged)
    }

    /// MetadataMap을 사용하여 필드 값을 타겟에 맞게 치환합니다.
    fn apply_mapping(&self, base: &mut Value, map: &MetadataMap) -> Result<()> {
        if let Some(obj) = base.as_object_mut() {
            for (field, value) in obj.iter_mut() {
                // description 필드는 매핑에서 제외
                if field == "description" {
                    continue;
                }

                // 문자열 타입 데이터만 매핑 적용
                if let Some(val_str) = value.as_str()
                    && let Some(field_mappings) = map.mappings.get(field)
                    && let Some(target_mappings) = field_mappings.get(val_str)
                    && let Some(mapped_val) = target_mappings.get(&self.target)
                {
                    *value = Value::String(mapped_val.clone());
                }
            }
        }
        Ok(())
    }

    /// 외부 YAML 데이터를 병합합니다. (타겟 전용 섹션 오버라이트)
    fn apply_external_override(&self, base: &mut Value, external: &Value) -> Result<()> {
        if !base.is_object() {
            *base = json!({});
        }

        let base_obj = base.as_object_mut().unwrap();
        let reserved_keys = BuildTarget::all_reserved_keys();

        if let Some(ext_obj) = external.as_object() {
            // 외부 파일의 최상위 키 검증 (예약어만 허용)
            for k in ext_obj.keys() {
                if !reserved_keys.contains(&k.as_str()) {
                    anyhow::bail!(
                        "External metadata contains unauthorized top-level field: '{}'. 
                         Only target reserved keys ({:?}) are allowed.",
                        k,
                        reserved_keys
                    );
                }
            }

            // 타겟 전용 필드들로 최종 오버라이트 (Shallow merge)
            let target_key = self.target.reserved_key();
            if let Some(target_section) = ext_obj.get(target_key).and_then(|v| v.as_object()) {
                for (k, v) in target_section {
                    base_obj.insert(k.clone(), v.clone());
                }
            }
        }
        Ok(())
    }

    /// 결과물에서 모든 타겟 섹션 예약어 키들을 제거합니다.
    fn cleanup(&self, base: &mut Value) {
        if let Some(base_obj) = base.as_object_mut() {
            for key in BuildTarget::all_reserved_keys() {
                base_obj.remove(*key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_merge_metadata_target_only_override() -> Result<()> {
        let merger = MetadataMerger::new(BuildTarget::GeminiCli, None);
        let base = json!({
            "name": "my-agent",
            "model": "default-model",
            "temperature": 0.5
        });
        let external = json!({
            "gemini-cli": {
                "model": "gemini-3.0-pro",
                "temperature": 0.2
            },
            "claude-code": {
                "model": "claude-3-opus"
            }
        });

        let result = merger.merge(&base, Some(&external))?;

        assert_eq!(result["name"], "my-agent");
        assert_eq!(result["model"], "gemini-3.0-pro");
        assert_eq!(result["temperature"], 0.2);
        assert!(result.get("gemini-cli").is_none());
        assert!(result.get("claude-code").is_none());
        Ok(())
    }

    #[test]
    fn test_apply_mapping() -> Result<()> {
        let mut mappings = HashMap::new();
        let mut model_map = HashMap::new();
        let mut sonnet_map = HashMap::new();
        sonnet_map.insert(BuildTarget::GeminiCli, "gemini-flash".to_string());
        model_map.insert("sonnet".to_string(), sonnet_map);
        mappings.insert("model".to_string(), model_map);

        let map = MetadataMap { mappings };
        let merger = MetadataMerger::new(BuildTarget::GeminiCli, Some(&map));

        let base = json!({
            "model": "sonnet",
            "description": "sonnet model"
        });

        let result = merger.merge(&base, None)?;

        assert_eq!(result["model"], "gemini-flash");
        assert_eq!(result["description"], "sonnet model"); // description should not be mapped
        Ok(())
    }

    #[test]
    fn test_merge_priority() -> Result<()> {
        let mut mappings = HashMap::new();
        let mut model_map = HashMap::new();
        let mut sonnet_map = HashMap::new();
        sonnet_map.insert(BuildTarget::GeminiCli, "gemini-flash".to_string());
        model_map.insert("sonnet".to_string(), sonnet_map);
        mappings.insert("model".to_string(), model_map);

        let map = MetadataMap { mappings };
        let merger = MetadataMerger::new(BuildTarget::GeminiCli, Some(&map));

        // FM -> Map -> External
        let base = json!({
            "model": "sonnet",
            "temperature": 0.5
        });

        // External should override Map result
        let external = json!({
            "gemini-cli": {
                "model": "gemini-pro-override"
            }
        });

        let result = merger.merge(&base, Some(&external))?;

        assert_eq!(result["model"], "gemini-pro-override");
        assert_eq!(result["temperature"], 0.5);
        Ok(())
    }
}
