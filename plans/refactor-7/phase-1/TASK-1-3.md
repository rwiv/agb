# TASK 1-3: `src/transformer` 모듈 내 상수 적용 및 타입 임포트 정리

## 1. 개요
`src/transformer` 모듈에서 출력 경로 결정 등에 사용되는 리터럴들을 상수로 교체하고, 타입 임포트 경로를 정리합니다.

## 2. 작업 내용
- `src/transformer/default.rs`: `commands`, `agents`, `skills`, `SKILL.md` 리터럴을 상수로 교체합니다.
- `src/transformer/gemini.rs`: `commands`, `GEMINI.md` 리터럴을 상수로 교체합니다.
- `src/transformer/mod.rs`: `CLAUDE.md`, `OPENCODE.md` 관련 로직이 있다면 상수를 사용하도록 수정합니다.
- 모든 `transformer` 모듈 파일 내에서 `crate::core::ResourceData` 등을 `use` 문을 사용하도록 수정합니다.

## 3. 검증 전략
- **빌드 검증:** `cargo check` 실행.
- **유닛 테스트:** `src/transformer/gemini.rs` 및 `src/transformer/default.rs` 내의 경로 생성 테스트가 통과하는지 확인합니다.
