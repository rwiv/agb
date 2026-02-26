# TASK-3-2: 다양한 타겟(Gemini, Claude)에서의 통합 동기화 테스트 수행

## 목적
서로 다른 변환 규격을 가진 에이전트 타겟들로부터의 역변환 및 동기화 공정이 실질적으로 통합 동작하는지 확인합니다.

## 작업 내용

1. **통합 테스트 작성 (`tests/syncer_integration_test.rs` 신설 또는 확장)**
   - `Gemini` 타겟 (TOML 커맨드)에서 수정한 내용이 마크다운 소스로 복원되는지 테스트.
   - `Claude` 타겟 (Frontmatter MD)에서 수정한 내용이 마크다운 소스의 Frontmatter를 깨뜨리지 않고 반영되는지 테스트.
   - `Skill` 리소스 내의 `extras` 파일 추가/삭제가 실제 파일 시스템에 반영되는지 E2E 테스트.

## 검증 방법
- `cargo test --test syncer_integration_test` 실행.
- 실제 프로젝트 구조를 모방한 fixture를 사용하여 `agb sync` 시뮬레이션.
