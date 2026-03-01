# TASK 1-1: `src/core/mod.rs`에 상수 정의 및 `src/core` 내 타입 임포트 정리

## 1. 개요
프로젝트 전체에서 공용으로 사용할 수 있는 파일 및 디렉터리 이름들을 `src/core/mod.rs`에 상수로 정의합니다. 또한 `src/core` 내의 타입 참조 방식을 정리합니다.

## 2. 작업 내용
- `src/core/mod.rs` 상단에 다음 상수들을 정의하고, `pub`으로 공개합니다:
  - 파일 이름: `AGENTS_MD`, `SKILL_MD`, `GEMINI_MD`, `CLAUDE_MD`, `AGENTS_MD`
  - 디렉터리 이름: `DIR_COMMANDS`, `DIR_AGENTS`, `DIR_SKILLS`
  - 타겟 이름: `TARGET_GEMINI`, `TARGET_CLAUDE`, `TARGET_OPENCODE`
  - 확장자: `EXT_MD`, `EXT_TOML`, `EXT_YAML`, `EXT_YML`
- `src/core/model.rs` 파일 내에서 자기 자신 또는 다른 타입을 참조할 때 `crate::core::ResourceData`와 같은 전체 경로를 `use` 문을 사용하도록 수정합니다.

## 3. 검증 전략
- **빌드 검증:** 상수 정의 후 프로젝트가 문제없이 빌드되는지 확인합니다. (`cargo check`)
- **타입 가독성:** `ResourceData` 등의 타입이 짧은 이름으로 사용되고 있는지 확인합니다.
