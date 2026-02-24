# Task 1.1: TransformedFile 이동 및 transformers 의존성 제거

## 1. Objective (목표)

- `src/transformers/base.rs`에 정의된 `TransformedFile` 구조체를 `src/resource/resource.rs`로 이동합니다.
- `transformers` 모듈이 더 이상 `TransformedFile`을 직접 소유하지 않게 함으로써, "리소스" 도메인에 결과물 모델을 집중시킵니다.
- `emitter`가 `transformers` 모듈을 의존하는 현재의 구조적 결합을 끊어냅니다.

## 2. Context & Files (작업 범위)

- **수정할 파일:**
  - `src/resource/resource.rs`: `TransformedFile` 구조체 정의 추가.
  - `src/transformers/base.rs`: `TransformedFile` 정의 제거 및 `crate::resource::TransformedFile` 참조로 변경.
  - `src/transformers/mod.rs`: `TransformedFile` re-export 수정.

## 3. Instructions (세부 지침)

### Step 1: `resource.rs`로 모델 이동

`src/resource/resource.rs`에 아래 구조체를 추가합니다. (기존 `transformers/base.rs`에서 복사)

```rust
#[derive(Debug, Clone)]
pub struct TransformedFile {
    pub path: PathBuf,
    pub content: String,
}
```

### Step 2: `transformers` 모듈 수정

- `src/transformers/base.rs`에서 `TransformedFile` 정의를 삭제합니다.
- `Transformer` 트레이트의 반환 타입을 `crate::resource::TransformedFile`로 명시하거나 임포트하여 사용합니다.

```rust
// src/transformers/base.rs 예시
use crate::resource::TransformedFile;

pub trait Transformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

### Step 3: `mod.rs` 정리

- `src/transformers/mod.rs`에서 더 이상 `base::TransformedFile`을 re-export 하지 않도록 수정하거나, `resource`에서 가져온 것을 다시 내보낼지 결정합니다. (의존성 단순화를 위해 `resource`에서 직접 가져오는 것을 권장)

## 4. Acceptance Criteria (검증 체크리스트)

1. `src/resource/resource.rs`에 `TransformedFile`이 정의되어 있는가?
2. `transformers` 모듈의 소스 코드에서 `TransformedFile` 정의가 제거되었는가?
3. `cargo check` 실행 시 타입 불일치 에러가 없는가?
