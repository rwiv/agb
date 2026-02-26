# Task 1.1: BuildTarget 이동 및 임포트 수정

## 1. Objective (목표)

- `builder`와 `transformer` 모듈 간의 순환 의존성(Circular Dependency)을 제거합니다.
- `BuildTarget` 열거형을 `builder::config`에서 `resource::model`로 이동시킵니다.
- `transformer` 모듈이 `builder`를 의존하지 않고 `resource`를 의존하도록 임포트 경로를 수정합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `src/main.rs` (전체 모듈 구조 확인)
- **수정할 파일:**
  - `src/resource/model.rs` (`BuildTarget` 정의 추가)
  - `src/resource/mod.rs` (`BuildTarget`을 `pub use`로 노출)
  - `src/transformer/mod.rs` (임포트 경로 수정: `builder::config::BuildTarget` -> `resource::BuildTarget`)
  - `src/builder/config.rs` (`BuildTarget` 정의 제거 및 `resource::BuildTarget` 임포트)

## 3. Instructions (세부 지침)

### Step 1: `BuildTarget` 정의 이동 (`src/resource/model.rs`)

`src/builder/config.rs`에 정의된 `BuildTarget` 열거형을 `src/resource/model.rs`로 옮깁니다. `serde::Deserialize` 파생(derive)이 필요하므로 관련 임포트를 확인하세요.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildTarget {
    #[serde(rename = "gemini-cli")]
    GeminiCli,
    #[serde(rename = "claude-code")]
    ClaudeCode,
    #[serde(rename = "opencode")]
    OpenCode,
}
```

### Step 2: `resource` 모듈에서 `BuildTarget` 노출 (`src/resource/mod.rs`)

`src/resource/mod.rs` 파일에 아래 내용을 추가하여 다른 모듈에서 편리하게 접근할 수 있도록 합니다.

```rust
pub use model::BuildTarget;
```

### Step 3: `transformer` 모듈 임포트 수정 (`src/transformer/mod.rs`)

`src/transformer/mod.rs`의 상단 임포트 구문을 수정하여 `builder` 모듈에 대한 의존성을 제거합니다.

- **Old:** `use crate::builder::config::BuildTarget;`
- **New:** `use crate::resource::BuildTarget;`

### Step 4: `builder::config` 모듈 수정 (`src/builder/config.rs`)

`src/builder/config.rs` 내의 `BuildTarget` 정의를 제거하고, `resource` 모듈에서 가져오도록 수정합니다.

- **Add:** `use crate::resource::BuildTarget;`
- **Remove:** `pub enum BuildTarget { ... }`

## 4. Constraints (제약 사항 및 금지 행동)

- `BuildTarget`의 각 항목에 대한 `serde(rename)` 속성을 유지해야 합니다.
- `BuildTarget`을 `resource` 모듈로 옮기는 것 외에 다른 로직을 수정하지 마세요. (Partial Update)

## 5. Acceptance Criteria (검증 체크리스트)

1. `cargo check` 실행 시 순환 의존성 관련 에러나 컴파일 에러가 없는가?
2. `src/transformer/mod.rs`에서 `builder` 모듈을 참조하는 `use` 구문이 완전히 제거되었는가?
3. `Config` 구조체가 여전히 정상적으로 파싱되는가? (특히 `target` 필드)
