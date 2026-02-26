# Plan: Syncer Refactoring (Patcher, Planner, Syncer)

이 문서는 `src/syncer/diff.rs`를 `MarkdownPatcher`, `SyncPlanner`, `SkillSyncer`로 분할하여 리팩토링하는 전체 과정을 관리합니다.

## Phase 1: `MarkdownPatcher` 기반 구축 (Foundation & Patcher)
*   **Task 1.1: `src/syncer/diff/markdown.rs` 생성 및 `MarkdownPatcher` 구현**
    *   *내용:* `src/syncer/diff.rs`의 `update_description`, `replace_content`, `diff_content` 함수를 `MarkdownPatcher` 구조체로 통합 및 리팩토링한다.
    *   *성공 기준:* 기존 단위 테스트 및 `MarkdownPatcher` 전용 테스트가 통과한다.

## Phase 2: 변경 탐지 및 계획 로직 분리 (Detection & Planner)
*   **Task 2.1: `src/syncer/diff/planner.rs` 생성 및 `SyncPlanner` 구현**
    *   *내용:* 두 디렉터리를 비교하여 `SyncAction` 목록을 반환하는 로직을 구현한다. (기존 `sync_skill_dir`의 탐지 로직 이주)
    *   *성공 기준:* 파일 시스템 모의(Mocking) 혹은 임시 디렉터리를 이용한 비교 테스트가 통과한다.

## Phase 3: 통합 및 동기화 조율 (Integration & Syncer)
*   **Task 3.1: `src/syncer/diff/mod.rs` 생성 및 `SkillSyncer` 구현**
    *   *내용:* `SyncPlanner`로부터 계획을 받아 실제 파일 작업을 수행하는 오케스트레이터를 구현하고, `sync_skill_dir` 함수를 이 구조체로 대체한다.
    *   *성공 기준:* 기존 `sync_skill_dir` 호출부와의 호환성을 유지하며 빌드에 성공한다.
*   **Task 3.2: 기존 `src/syncer/diff.rs` 제거 및 최종 검증**
    *   *내용:* `src/syncer/diff.rs` 파일을 삭제하고 모든 동기화 관련 E2E 테스트를 수행한다.
    *   *성공 기준:* `agb sync` 명령어가 정상적으로 동작하며 모든 테스트를 통과한다.
