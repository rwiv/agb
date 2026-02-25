# Transformer Module

`transformer` 모듈은 `agb`의 내부 리소스 모델(`Resource`)을 각 에이전트(Gemini, Claude, OpenCode)의 규격에 맞는 물리적 파일 형식으로 변환하는 역할을 담당합니다.

## 핵심 역할

1. **포맷 변환**: JSON 메타데이터와 Markdown 컨텐츠를 타겟 에이전트가 이해할 수 있는 형식(예: TOML, Frontmatter Markdown)으로 변환합니다.
2. **경로 결정**: 각 리소스가 타겟 에이전트의 파일 시스템 구조에서 어디에 위치해야 하는지 정의합니다. (예: `commands/foo.toml`)
3. **전역 지침 처리**: 루트의 `AGENTS.md`를 각 에이전트의 메인 메모리 파일(Gemini: `GEMINI.md`, Claude/OpenCode: `CLAUDE.md`)로 변환합니다.

## 모듈 구조

- `mod.rs`: `Transformer` 트레이트 및 `TransformerFactory` 정의.
- `gemini.rs`: Gemini-cli용 하이브리드 변환기 (Commands는 TOML, 나머지는 DefaultTransformer 사용).
- `default.rs`: 공용 마크다운 변환기 (Claude-code, OpenCode 및 Gemini의 Agents/Skills 처리).

## 주요 구성 요소

### 1. `Transformer` Trait
모든 변환기가 구현해야 하는 인터페이스입니다.

```rust
pub trait Transformer {
    /// 리소스를 타겟 포맷으로 변환
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침(AGENTS.md)을 타겟 규격으로 변환
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

### 2. `TransformerFactory`
`BuildTarget` 열거형을 기반으로 적절한 `Transformer` 구현체를 동적으로 생성(Boxed trait object)하여 반환합니다.

## 타겟별 특이사항

- **Gemini-cli**: 
  - **Commands**: 메타데이터 -> TOML Key-Value 매핑, 본문 -> TOML `prompt` 필드로 삽입하여 `*.toml` 생성.
  - **Agents / Skills**: `DefaultTransformer`를 사용하여 메타데이터가 포함된 마크다운 `*.md` 생성.
  - 전역 지침: `GEMINI.md` 생성.
- **Claude-code / OpenCode**: 
  - `DefaultTransformer` 사용. 메타데이터 -> YAML Frontmatter, 본문 -> 마크다운 본문 결합하여 `*.md` 생성.
  - 전역 지침: `CLAUDE.md` 또는 `OPENCODE.md` 생성.

## 새로운 에이전트 추가 방법

1. `src/transformer/` 내에 새로운 모듈을 생성합니다 (예: `new_agent.rs`).
2. `Transformer` 트레이트를 구현합니다.
3. `TransformerFactory::create` 함수에 해당 에이전트 분기를 추가합니다.
4. `src/builder/config.rs`의 `BuildTarget` 열거형에 새로운 에이전트 이름을 등록합니다.
