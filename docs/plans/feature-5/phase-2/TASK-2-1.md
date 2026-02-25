# Task 2.1: GeminiTransformer 위임 로직 구현

## 개요
`GeminiTransformer`가 모든 리소스를 TOML로 바꾸던 방식에서 벗어나, `Command` 외의 리소스는 `DefaultTransformer`를 사용하도록 수정합니다.

## 상세 작업 내용

1.  **`src/transformer/gemini.rs` 수정**:
    - `GeminiTransformer` 내부에서 `DefaultTransformer`를 사용하도록 변경.
2.  **분기 로직 적용**:
    - `Resource::Command` -> 기존 TOML 변환 로직 유지.
    - `Resource::Agent` / `Resource::Skill` -> `DefaultTransformer::transform` 호출.
3.  **루트 프롬프트 처리**:
    - `transform_root_prompt`는 여전히 `GEMINI.md`를 반환하도록 유지 (이미 `DefaultTransformer`에서 지원 예정).

## 성공 기준
- `GeminiTransformer`에서 `Command`는 `.toml`로, `Agent`/`Skill`은 `.md`로 변환된다.
