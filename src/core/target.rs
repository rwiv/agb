use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const TARGET_GEMINI: &str = "gemini-cli";
pub const TARGET_CLAUDE: &str = "claude-code";
pub const TARGET_OPENCODE: &str = "opencode";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildTarget {
    #[serde(rename = "gemini-cli")]
    GeminiCli,
    #[serde(rename = "claude-code")]
    ClaudeCode,
    #[serde(rename = "opencode")]
    OpenCode,
}

impl BuildTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            BuildTarget::GeminiCli => TARGET_GEMINI,
            BuildTarget::ClaudeCode => TARGET_CLAUDE,
            BuildTarget::OpenCode => TARGET_OPENCODE,
        }
    }

    pub fn reserved_key(&self) -> &'static str {
        self.as_str()
    }

    pub fn all_reserved_keys() -> &'static [&'static str] {
        &[TARGET_GEMINI, TARGET_CLAUDE, TARGET_OPENCODE]
    }

    /// 두 개의 메타데이터 객체를 타겟 규칙에 따라 병합합니다.
    pub fn merge_metadata(&self, base: &mut Value, external: &Value) {
        if !base.is_object() {
            *base = json!({});
        }

        let base_obj = base.as_object_mut().unwrap();
        let reserved_keys = Self::all_reserved_keys();

        if let Some(ext_obj) = external.as_object() {
            // 1. 외부 파일의 일반 필드들을 base에 덮어씀 (Shallow merge)
            for (k, v) in ext_obj {
                if !reserved_keys.contains(&k.as_str()) {
                    base_obj.insert(k.clone(), v.clone());
                }
            }

            // 2. 타겟 전용 필드들로 최종 오버라이트
            let target_key = self.reserved_key();
            if let Some(target_section) = ext_obj.get(target_key).and_then(|v| v.as_object()) {
                for (k, v) in target_section {
                    base_obj.insert(k.clone(), v.clone());
                }
            }
        }

        // 3. 결과물에서 모든 타겟 섹션 예약어 키들 제거
        for key in reserved_keys {
            base_obj.remove(*key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_metadata_basic() {
        let target = BuildTarget::GeminiCli;
        let mut base = json!({ "a": 1, "b": 2 });
        let external = json!({ "b": 3, "c": 4 });

        target.merge_metadata(&mut base, &external);

        assert_eq!(base["a"], 1);
        assert_eq!(base["b"], 3);
        assert_eq!(base["c"], 4);
    }

    #[test]
    fn test_merge_metadata_with_target_override() {
        let target = BuildTarget::GeminiCli;
        let mut base = json!({
            "model": "default",
            "temp": 0.5
        });
        let external = json!({
            "temp": 0.8,
            "gemini-cli": {
                "model": "gemini-pro",
                "temp": 0.2
            },
            "claude-code": {
                "model": "claude-opus"
            }
        });

        target.merge_metadata(&mut base, &external);

        assert_eq!(base["model"], "gemini-pro");
        assert_eq!(base["temp"], 0.2);
        assert!(base.get("gemini-cli").is_none());
        assert!(base.get("claude-code").is_none());
    }

    #[test]
    fn test_all_reserved_keys() {
        let keys = BuildTarget::all_reserved_keys();
        assert!(keys.contains(&"gemini-cli"));
        assert!(keys.contains(&"claude-code"));
        assert!(keys.contains(&"opencode"));
    }
}
