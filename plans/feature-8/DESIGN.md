# DESIGN: Type-Aware Registry & Core Relocation

## 1. 개요 (Overview)
현재 `Registry`는 리소스의 타입에 관계없이 이름만으로 중복을 체크합니다. 또한 `Registry`는 빌더뿐만 아니라 시스템 전반에서 리소스 관리를 위해 사용되는 핵심 컴포넌트이므로, 이를 `core` 모듈로 이동하여 의존성 구조를 개선하고 타입 기반의 중복 체크 기능을 추가합니다.

## 2. 변경 사항 (Changes)

### 2.1. 데이터 모델 확장 (`src/core/model.rs`)
- `ResourceType` 열거형 추가 (`Command`, `Agent`, `Skill`).
- `Resource` 열거형에 `r_type()` 메서드를 추가하여 현재 리소스의 타입을 반환하도록 함.
- `ResourceType`은 `Display` 트레이트를 구현하여 에러 메시지 출력 시 활용.

### 2.2. Registry 이동 및 고도화 (`src/core/registry.rs`)
- `src/builder/registry.rs` 파일을 `src/core/registry.rs`로 이동.
- `Registry` 내부의 `HashMap` 키를 `String`에서 `(ResourceType, String)` 조합으로 변경.
- `register()` 메서드가 타입과 이름을 동시에 고려하여 중복을 체크하도록 수정.
- 동일한 이름이라도 타입이 다르면(예: `command/write-plan` vs `skill/write-plan`) 공존을 허용함.

### 2.3. 의존성 및 임포트 정리
- `src/builder/mod.rs` 및 관련 파일에서 `Registry` 임포트 경로 수정.
- `src/core/mod.rs`에서 `registry` 모듈을 노출.

## 3. 상세 설계 (Detailed Design)

### 3.1. ResourceType 정의
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Command,
    Agent,
    Skill,
}
```

### 3.2. Registry 키 구조 변경
```rust
pub struct Registry {
    /// Key: (ResourceType, Name)
    resources: HashMap<(ResourceType, String), Resource>,
}
```

## 4. 기대 효과
- **유연성 증가**: 동일한 이름의 리소스를 다른 타입으로 정의할 수 있음 (예: 특정 기능을 수행하는 command와 이를 설명하는 skill의 명칭 통일).
- **구조적 개선**: 리소스 관리의 핵심인 `Registry`가 `core`에 위치함으로써 아키텍처 정합성 확보.
