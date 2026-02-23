# Transformers Module

`transformers` 모듈은 `agb`의 내부 리소스 모델(`Resource`)을 각 에이전트(Gemini, Claude 등)의 규격에 맞는 물리적 파일 형식으로 변환하는 역할을 담당합니다.

## 핵심 역할

1. **포맷 변환**: JSON 메타데이터와 Markdown 컨텐츠를 타겟 에이전트가 이해할 수 있는 형식(예: TOML, Frontmatter Markdown)으로 변환합니다.
2. **경로 결정**: 각 리소스가 타겟 에이전트의 파일 시스템 구조에서 어디에 위치해야 하는지 정의합니다.
3. **전역 지침 처리**: 루트의 `AGENTS.md`를 각 에이전트의 메인 메모리 파일(예: `GEMINI.md`)로 변환합니다.

## 주요 구성 요소

### 1. `Transformer` Trait
모든 변환기가 구현해야 하는 인터페이스입니다.

```rust
pub trait Transformer {
    /// 리소스를 타겟 포맷으로 변환
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침을 타겟 규격으로 변환
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

### 2. `TransformedFile`
변환 결과를 담는 구조체로, `Emitter` 모듈에서 파일로 쓰기 위해 사용됩니다.

- `path`: 결과물이 저장될 상대 경로 (예: `commands/foo.toml`)
- `content`: 변환된 파일의 실제 내용

## 지원되는 에이전트

- **Gemini-cli (`gemini-cli`)**: 리소스를 TOML 형식으로 변환하며, `commands/`, `agents/`, `skills/` 폴더 구조를 사용합니다.
- **Claude-code (`claude-code`)**: 마크다운 기반의 변환을 지원할 예정입니다 (현재 인터페이스만 존재).

## 새로운 에이전트 추가 방법

1. `src/transformers/` 내에 새로운 모듈을 생성합니다 (예: `new_agent.rs`).
2. `Transformer` 트레이트를 구현하는 구조체를 정의합니다.
3. `src/transformers/mod.rs`의 `get_transformer` 팩토리 함수에 해당 에이전트 분기를 추가합니다.
4. `src/config.rs`의 `BuildTarget` 열거형에 새로운 에이전트 이름을 등록합니다.

## 변환 규칙 (Gemini-cli 예시)

- **Markdown**: TOML의 `prompt` 필드로 삽입됩니다.
- **JSON Metadata**: TOML의 최상위 키-값 쌍으로 매핑됩니다.
- **Skills**: `skills/[name]/[name].toml` 경로로 생성되어 디렉터리 기반 관리를 지원합니다.
