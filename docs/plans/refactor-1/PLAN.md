# Plan: agb Refactoring (Structure & Naming)

이 마스터 리스트는 프로젝트의 아키텍처 정제 및 모듈 네이밍 개선을 위한 태스크를 관리합니다.

## Phase 1: 아키텍처 정제 및 모듈 네이밍 개선
*   **Task 1.1: `core` 모듈을 `resource`로 리네이밍**
    *   *성공 기준:* 모든 `core` 모듈 참조가 `resource`로 변경되고, 빌드 및 테스트가 성공한다.
*   **Task 1.2: `fs_utils`를 최상위 `utils` 모듈로 분리**
    *   *성공 기준:* `emitter/fs_utils.rs` 로직이 `utils/fs.rs`로 이동하고, `emitter`를 포함한 타 모듈에서 정상적으로 참조된다.
