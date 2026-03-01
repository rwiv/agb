# TASK-2-1: Documentation update

전체 프로젝트 문서와 계획 문서에서 `OPENCODE.md`를 `AGENTS.md`로 수정합니다.

## 수정 대상 목록

1.  **메인 문서**: `README.md`, `specs/prd.md`, `specs/format.md`, `specs/spec.md`, `specs/design.md`.
2.  **모듈 문서**: `src/transformer/README.md`.
3.  **기존 계획 문서들**:
    - `plans/refactor-4/phase-1/TASK-1-1.md`
    - `plans/feature-5/phase-1/TASK-1-1.md`
    - `plans/feature-5/PLAN.md`
    - `plans/feature-10/DESIGN.md`
    - `plans/feature-10/phase-3/TASK-3-4.md`
    - `plans/feature-1/phase-5/TASK-5-2.md`
    - `plans/refactor-7/DESIGN.md`
    - `plans/refactor-7/phase-1/TASK-1-3.md`
    - `plans/refactor-7/phase-1/TASK-1-1.md`

## 수정 방식
- `OPENCODE.md` 문자열을 `AGENTS.md`로 일괄 치환합니다.
- `OPENCODE_MD` 상수 이름이 언급된 경우 `AGENTS_MD`로 수정합니다.

## 검증 방법
- `grep` 등을 통해 `OPENCODE.md` 혹은 `OPENCODE_MD`가 남아있는지 확인합니다.
