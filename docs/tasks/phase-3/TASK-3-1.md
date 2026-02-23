# Task 3.1: Transformer 트레이트 정의

## 1. Objective (목표)

- 에이전트별(Gemini, Claude 등)로 서로 다른 변환 로직을 일관된 인터페이스로 다루기 위한 `Transformer` 트레이트를 정의합니다.
- 변환 결과를 담는 공통 구조체(`TransformedFile`)를 설계하여 이후 단계의 `Emitter`가 타겟 에이전트에 구애받지 않고 파일을 쓸 수 있도록 합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `docs/specs/TECH_SPEC.md` (Transformer 트레이트 설계 확인)
  - `src/core/resource.rs` (Resource 모델 구조 확인)
- **생성 및 수정할 파일:**
  - `src/transformers/mod.rs` (신규 생성: 트레이트 및 공통 모델 정의)

## 3. Instructions (세부 지침)

### Step 1: `TransformedFile` 구조체 정의

변환된 파일의 경로와 내용을 담는 구조체를 정의하세요.

- `path`: `PathBuf` 형식 (결과물이 저장될 상대 경로, 예: `commands/foo.toml`)
- `content`: `String` 형식 (변환이 완료된 파일의 실제 내용)

### Step 2: `Transformer` 트레이트 정의

`TECH_SPEC.md`의 설계를 바탕으로 아래 인터페이스를 구현하세요.

```rust
use anyhow::Result;
use crate::core::resource::Resource;

pub trait Transformer {
    /// 개별 리소스(Command, Agent, Skill)를 타겟 포맷으로 변환합니다.
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침(AGENTS.md)을 타겟 규격의 메인 메모리 파일로 변환합니다.
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

### Step 3: 에러 타입 정의 (선택 사항)

- 변환 과정에서 발생할 수 있는 특수한 에러(예: 지원하지 않는 리소스 타입 등)를 `thiserror`를 사용하여 정의하는 것을 고려하세요.

## 4. Constraints (제약 사항 및 금지 행동)

- 특정 에이전트(Gemini 등)에 종속된 로직을 이 트레이트에 직접 작성하지 마세요. 오직 추상화된 인터페이스만 정의합니다.
- `src/transformers/mod.rs`에서 `Transformer` 트레이트를 `pub`으로 노출하여 외부 모듈에서 접근 가능하게 하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `Transformer` 트레이트가 `TECH_SPEC.md`에 명시된 메서드 시그니처를 정확히 포함하고 있는가?
2. `TransformedFile` 구조체가 파일 경로와 내용을 모두 포함하고 있는가?
3. `src/transformers/mod.rs` 파일이 에러 없이 컴파일되는가?
4. `Resource` 객체를 인자로 받아 `Result<TransformedFile>`을 반환하는 구조가 확립되었는가?
