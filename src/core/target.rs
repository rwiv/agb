use serde::{Deserialize, Serialize};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_reserved_keys() {
        let keys = BuildTarget::all_reserved_keys();
        assert!(keys.contains(&"gemini-cli"));
        assert!(keys.contains(&"claude-code"));
        assert!(keys.contains(&"opencode"));
    }
}
