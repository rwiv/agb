# TASK-3-1: Verification

테스트 코드를 업데이트하고 전체 테스트를 실행하여 변경 사항이 정상적으로 반영되었는지 확인합니다.

## 작업 내용

### 1. `src/transformer/default.rs` 단위 테스트 수정
- `test_default_root_prompt_transformation` 테스트에서 `BuildTarget::OpenCode` 시 `OPENCODE_MD` 대신 `AGENTS_MD`를 확인하도록 수정.

### 2. 전체 테스트 실행
- `cargo test`를 실행하여 모든 테스트(단위 테스트, 통합 테스트, E2E 테스트)가 통과하는지 확인.
- 특히 E2E 테스트에서 `OPENCODE.md`가 생성되지 않고 `AGENTS.md`가 생성되는지 확인.

## 검증 방법
- `cargo test`의 성공 여부를 확인합니다.
