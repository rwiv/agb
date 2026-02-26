# Plan: Skill Metadata Renaming (Domain-specific Naming)

스킬(Skill) 리소스의 메타데이터 파일명을 고정된 `METADATA.yaml`에서 스킬 이름과 일치하는 `[skill_name].yaml` 방식으로 변경하여, 미래의 표준 규격(agentskills.io 등)과의 충돌을 방지하고 일관성을 확보합니다.

## Phase 1: Core Logic & Documentation

- [ ] **TASK-1-1: Resource Loader 로직 수정**
  - `src/resource/loader.rs`에서 스킬 폴더 탐색 시 폴더명과 동일한 이름의 메타데이터 파일(`.json`, `.yaml`, `.yml`)을 찾도록 수정.
  - 기존 `METADATA.*` 로직 제거.
- [ ] **TASK-1-2: 시스템 문서 업데이트**
  - `README.md`, `specs/SYSTEM_SPEC.md`, `src/resource/README.md` 등에서 변경된 명명 규칙 반영.
- [ ] **TASK-1-3: 테스트 코드 및 검증**
  - `src/resource/loader.rs` 내 유닛 테스트 수정.
  - `tests/e2e_build_test.rs` 및 테스트용 피처(`tests/fixtures`) 업데이트 및 전체 빌드 검증.
