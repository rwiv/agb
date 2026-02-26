# PLAN: 멀티라인 Description 동기화 방지 구현

## 개요
멀티라인 `description` 필드가 감지될 경우 동기화를 중단하여 원본 소스의 파손을 방지하는 기능을 구현합니다.

## Phase 1: MdPatcher 기능 강화
`MdPatcher` 구조체의 `update_description` 메서드를 수정하여 검증 로직을 추가합니다.

- [x] **TASK-1-1**: `update_description` 반환 타입 변경 및 검증 로직 구현.
- [x] **TASK-1-2**: 멀티라인 감지 시나리오에 대한 단위 테스트 추가.

## Phase 2: 호출부 대응 및 통합 검증
`Syncer`와 `SkillSyncer`에서 변경된 인터페이스를 적용하고 전체 흐름을 확인합니다.

- [x] **TASK-2-1**: `src/syncer/mod.rs` 및 `src/syncer/skill.rs` 수정.
- [x] **TASK-2-2**: 실제 멀티라인 데이터가 포함된 소스를 대상으로 `agb sync` 실패 여부 검증 (E2E 테스트).

## 검증 계획
1. **단위 테스트**: `patcher.rs` 내에서 다양한 멀티라인 케이스(마커 사용, 들여쓰기 사용, 입력값 줄바꿈)에 대해 `Err`가 발생하는지 확인.
2. **E2E 테스트**: `tests/` 디렉터리에 재현 케이스를 추가하여 `agb sync`가 패닉 없이 에러 메시지를 출력하며 종료되는지 확인.
