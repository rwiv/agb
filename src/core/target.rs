use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

impl FromStr for BuildTarget {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            TARGET_GEMINI => Ok(BuildTarget::GeminiCli),
            TARGET_CLAUDE => Ok(BuildTarget::ClaudeCode),
            TARGET_OPENCODE => Ok(BuildTarget::OpenCode),
            _ => anyhow::bail!("Unknown build target: {}", s),
        }
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

    #[test]
    fn test_from_str() {
        assert_eq!(BuildTarget::from_str("gemini-cli").unwrap(), BuildTarget::GeminiCli);
        assert_eq!(BuildTarget::from_str("claude-code").unwrap(), BuildTarget::ClaudeCode);
        assert_eq!(BuildTarget::from_str("opencode").unwrap(), BuildTarget::OpenCode);
        assert!(BuildTarget::from_str("unknown").is_err());
    }
}
