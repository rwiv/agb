# Task 2.2: Gemini 타겟 결과물 구조 검증

## 개요
Gemini 타겟으로 빌드했을 때의 최종 파일 시스템 구조와 내용을 검증합니다.

## 상세 작업 내용

1.  **유닛 테스트 추가**:
    - `gemini.rs`에 `Agent`와 `Skill`이 마크다운으로 변환되는지 확인하는 테스트 추가.
2.  **E2E 테스트 연동**:
    - 기존 `tests/e2e_build_test.rs` 등에서 Gemini 타겟의 기대 결과물 확장자를 업데이트.

## 성공 기준
- 빌드 결과물에 `agents/*.md`와 `commands/*.toml`이 공존한다.
- `skills/` 하위 리소스도 마크다운으로 정상 생성된다.
