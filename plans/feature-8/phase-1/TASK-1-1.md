# TASK-1-1: Core 모델 및 구조 변경

## 1. 작업 개요 (Summary)
`src/core/model.rs` 파일에 `ResourceType` 열거형을 정의하고, `Resource` 열거형에 타입을 반환하는 `r_type()` 메서드를 추가합니다.

## 2. 세부 구현 사항 (Implementation Details)

### 2.1. ResourceType 정의 추가
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Command,
    Agent,
    Skill,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResourceType::Command => "command",
            ResourceType::Agent => "agent",
            ResourceType::Skill => "skill",
        };
        write!(f, "{}", s)
    }
}
```

### 2.2. Resource::r_type() 메서드 추가
```rust
impl Resource {
    pub fn r_type(&self) -> ResourceType {
        match self {
            Resource::Command(_) => ResourceType::Command,
            Resource::Agent(_) => ResourceType::Agent,
            Resource::Skill(_) => ResourceType::Skill,
        }
    }
}
```

## 3. 검증 방법 (Verification)
- `cargo check`를 실행하여 컴파일 에러가 없는지 확인.
- `src/core/model.rs` 파일에 해당 정의가 올바르게 추가되었는지 확인.
