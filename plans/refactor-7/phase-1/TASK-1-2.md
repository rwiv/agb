# TASK 1-2: `src/builder` 및 `src/loader` 모듈 내 상수 적용 및 타입 임포트 정리

## 1. 개요
`src/builder` 및 `src/loader` 모듈에서 사용되는 모든 파일명 및 디렉터리 이름 리터럴을 `src/core`에 정의된 상수로 교체합니다. 또한 `crate::core::...` 전체 경로 참조를 `use` 문을 사용하도록 수정합니다.

## 2. 작업 내용
- `src/builder/emitter.rs`: `commands`, `agents`, `skills`, `GEMINI.md`, `CLAUDE.md`, `AGENTS.md` 리터럴을 상수로 교체합니다.
- `src/builder/mod.rs`: `AGENTS.md` 리터럴을 상수로 교체합니다.
- `src/loader/resolver.rs`: `commands`, `agents`, `skills` 리터럴을 상수로 교체합니다.
- `src/loader/parser.rs`: `commands`, `agents`, `skills` 리터럴을 상수로 교체합니다.
- 각 파일 상단에 `use crate::core::{Resource, ResourceData, ...};`를 추가하여 장황한 경로를 제거합니다.

## 3. 검증 전략
- **빌드 검증:** `cargo check` 및 `cargo test`를 통해 전체 정합성을 확인합니다.
- **코드 리뷰:** 각 모듈 내에서 리터럴이 상수로 잘 교체되었는지 확인합니다.
