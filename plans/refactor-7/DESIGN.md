# Refactor 7: 상수화 및 구조적 개선

## 1. 개요
프로젝트 전반에 산재한 매직 넘버(파일 및 디렉터리 이름)를 상수로 통합 관리하고, 중복된 타입 경로 및 유틸리티 함수를 정리하여 코드의 가독성과 유지보수성을 향상시킵니다.

## 2. 설계 상세

### 2.1 상수 관리 (`src/core/mod.rs` 또는 `constants.rs`)
다음과 같은 파일 및 디렉터리 이름들을 상수로 정의합니다.

```rust
// 예시: src/core/mod.rs (또는 constants.rs)
pub const AGENTS_MD: &str = "AGENTS.md";
pub const SKILL_MD: &str = "SKILL.md";
pub const GEMINI_MD: &str = "GEMINI.md";
pub const CLAUDE_MD: &str = "CLAUDE.md";
pub const OPENCODE_MD: &str = "OPENCODE.md";

pub const DIR_COMMANDS: &str = "commands";
pub const DIR_AGENTS: &str = "agents";
pub const DIR_SKILLS: &str = "skills";
```

### 2.2 타입 임포트 정리
`crate::core::ResourceData`와 같이 전체 경로를 사용하는 부분을 정리합니다.

- **대상 파일:** `src/transformer/gemini.rs`, `src/builder/registry.rs` 등
- **변경 방식:** 파일 상단에 `use crate::core::{Resource, ResourceData, BuildTarget};`를 추가하고 본문에서는 짧은 이름을 사용합니다.

### 2.3 유틸리티 함수 이동
`src/transformer/gemini.rs`에 위치한 `json_to_toml` 함수를 `src/utils/toml.rs`로 이동합니다.

- **경로:** `src/utils/toml.rs`
- **역할:** `serde_json::Value`를 `toml::Value`로 변환하는 범용 기능을 제공합니다.
- **의존성:** `anyhow`, `serde_json`, `toml`

## 3. 기대 효과
- **일관성:** 파일 이름이나 디렉터리 구조가 변경될 때 상숫값 하나만 수정하여 전체 프로젝트에 반영할 수 있습니다.
- **가독성:** 장황한 타입 경로가 제거되어 코드의 비즈니스 로직에 더 집중할 수 있습니다.
- **재사용성:** `json_to_toml` 기능이 분리되어 다른 모듈에서도 쉽게 활용할 수 있습니다.
