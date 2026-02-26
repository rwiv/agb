// 파일 이름 상수
pub const AGENTS_MD: &str = "AGENTS.md";
pub const SKILL_MD: &str = "SKILL.md";
pub const GEMINI_MD: &str = "GEMINI.md";
pub const CLAUDE_MD: &str = "CLAUDE.md";
pub const OPENCODE_MD: &str = "OPENCODE.md";

// 금지된 파일 목록 (플러그인 내부에 존재할 수 없음)
pub const FORBIDDEN_FILES: &[&str] = &[GEMINI_MD, CLAUDE_MD, OPENCODE_MD];

// 디렉터리 이름 상수
pub const DIR_COMMANDS: &str = "commands";
pub const DIR_AGENTS: &str = "agents";
pub const DIR_SKILLS: &str = "skills";

// 확장자 상수
pub const EXT_MD: &str = ".md";
pub const EXT_TOML: &str = ".toml";
pub const EXT_YAML: &str = ".yaml";
pub const EXT_YML: &str = ".yml";
