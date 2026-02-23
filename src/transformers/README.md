# Transformers Module

`transformers` 모듈은 `agb`의 내부 리소스 모델(`Resource`)을 각 에이전트(Gemini, Claude 등)의 규격에 맞는 물리적 파일 형식으로 변환하는 역할을 담당합니다.

## 핵심 역할

1. **포맷 변환**: JSON 메타데이터와 Markdown 컨텐츠를 타겟 에이전트가 이해할 수 있는 형식(예: TOML, Frontmatter Markdown)으로 변환합니다.
2. **경로 결정**: 각 리소스가 타겟 에이전트의 파일 시스템 구조에서 어디에 위치해야 하는지 정의합니다.
3. **전역 지침 처리**: 루트의 `AGENTS.md`를 각 에이전트의 메인 메모리 파일(예: `GEMINI.md`)로 변환합니다.

## 모듈 구조

- `base.rs`: `Transformer` 트레이트 및 `TransformedFile` 등 공통 인터페이스와 데이터 모델 정의.
- `mod.rs`: 각 변환기 모듈 선언 및 타겟에 맞는 변환기를 생성하는 `get_transformer` 팩토리 함수 제공.
- `gemini.rs`: Gemini-cli (TOML 기반) 변환기 구현.
- `claude.rs`: Claude-code (Markdown 기반) 변환기 구현.

## 주요 구성 요소

### 1. `Transformer` Trait (`base.rs`)
모든 변환기가 구현해야 하는 인터페이스입니다.

```rust
pub trait Transformer {
    /// 리소스를 타겟 포맷으로 변환
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침을 타겟 규격으로 변환
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

## 새로운 에이전트 추가 방법

1. `src/transformers/` 내에 새로운 모듈을 생성합니다 (예: `opencode.rs`).
2. `base::Transformer` 트레이트를 구현하는 구조체를 정의합니다.
3. `src/transformers/mod.rs`에서 새 모듈을 선언(`pub mod opencode;`)하고 `get_transformer` 함수에 해당 에이전트 분기를 추가합니다.
4. `src/config.rs`의 `BuildTarget` 열거형에 새로운 에이전트 이름을 등록합니다.
