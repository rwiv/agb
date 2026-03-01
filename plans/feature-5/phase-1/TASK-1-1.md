# Task 1.1: DefaultTransformer 정의 및 구현

## 개요
기존 `ClaudeTransformer`와 `OpenCodeTransformer`의 중복을 제거하기 위해, 범용 마크다운 변환기인 `DefaultTransformer`를 구현합니다.

## 상세 작업 내용

1.  **`src/transformer/default.rs` 생성**:
    - `DefaultTransformer` 구조체 정의.
    - `Target` 정보를 저장하거나 생성 시 전달받아 루트 파일명을 결정할 수 있도록 설계.
2.  **Frontmatter 구조 통합**:
    - 모든 메타데이터를 `metadata` 키 하위에 배치하는 구조로 통일.
    ```rust
    #[derive(Serialize, Deserialize)]
    struct DefaultFrontmatter {
        metadata: serde_json::Value,
    }
    ```
3.  **변환 로직 구현**:
    - `transform`: 메타데이터를 YAML Frontmatter로 변환하고 본문과 결합. `.md` 확장자로 경로 설정.
    - `transform_root_prompt`: 타겟 에이전트에 맞는 루트 파일명(`CLAUDE.md`, `AGENTS.md`, `GEMINI.md` 등) 반환.

## 성공 기준
- `DefaultTransformer`가 성공적으로 컴파일된다.
- `metadata` 키를 포함한 YAML Frontmatter가 생성된다.
- 타겟별로 정확한 루트 파일명이 생성된다.
