# Plan: Rename OpenCode Global Instruction Filename

OpenCode 타겟의 전역 지침 파일명을 `OPENCODE.md`에서 `AGENTS.md`로 변경합니다.
이를 위해 `OPENCODE_MD` 상수를 제거하고 `AGENTS_MD`를 사용하도록 코드를 수정하며, 관련 문서와 테스트를 업데이트합니다.

## Phase 1: Core Constants & Code modification
- [x] TASK-1-1: `src/core/constants.rs`, `src/transformer/default.rs`, `src/builder/emitter.rs` 수정.

## Phase 2: Documentation update
- [x] TASK-2-1: 메인 문서 및 기존 계획 문서들에서 `OPENCODE.md` 수정.

## Phase 3: Verification
- [x] TASK-3-1: 단위 테스트 수정 및 전체 테스트 실행.
