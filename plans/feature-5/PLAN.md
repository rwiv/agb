# Plan: Transformer Unified & Hybrid Gemini Support

이 문서는 `ClaudeTransformer`와 `OpenCodeTransformer`를 `DefaultTransformer`로 통합하고, `GeminiTransformer`에 하이브리드 변환 방식(Commands: TOML, Others: Markdown)을 도입하는 계획을 담고 있습니다.

## Phase 1: `DefaultTransformer` 구현 (Core Unification)
*   [x] **Task 1.1: `DefaultTransformer` 정의 및 구현**
    *   *성공 기준:* `metadata` 키 기반의 Frontmatter 구조를 가지며, 타겟별 루트 파일명(`CLAUDE.md`, `OPENCODE.md` 등)을 처리하는 통합 변환기를 구현한다.
*   [x] **Task 1.2: 기존 Claude/OpenCode 로직 이관 및 테스트**
    *   *성공 기준:* 기존 `claude.rs`, `opencode.rs`의 테스트 케이스를 `DefaultTransformer`로 성공적으로 이관하고 통과시킨다.

## Phase 2: Gemini 하이브리드 변환 적용 (Hybrid Gemini)
*   [x] **Task 2.1: `GeminiTransformer` 위임 로직 구현**
    *   *성공 기준:* `Resource::Command`는 기존 TOML 방식으로 처리하고, `Agent`와 `Skill`은 `DefaultTransformer`에 위임하여 처리한다.
*   [x] **Task 2.2: Gemini 타겟 결과물 구조 검증**
    *   *성공 기준:* `target: gemini-cli` 빌드 시 `commands/`는 `.toml`로, `agents/` 및 `skills/`는 `.md`로 생성됨을 확인한다.

## Phase 3: 정리 및 최종 통합 (Cleanup & Finalize)
*   [x] **Task 3.1: 기존 트랜스포머 파일 제거 및 모듈 업데이트**
    *   *성공 기준:* `claude.rs`, `opencode.rs`를 삭제하고 `src/transformer/mod.rs`를 새로운 구조에 맞게 수정한다.
*   [x] **Task 3.2: 전체 워크플로우 엔드투엔드(E2E) 테스트**
    *   *성공 기준:* 모든 지원 타겟(Gemini, Claude, OpenCode)에 대해 일관된 빌드 결과와 정상 작동을 확인한다.
