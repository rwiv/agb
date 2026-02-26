# 설계: BuildTarget 분리 및 결합도 낮추기 (Decoupling)

## 1. 문제 정의

현재 `src/loader/parser.rs`에 위치한 `ResourceParser`는 `TARGET_GEMINI`와 같은 특정 타겟 전용 키에 대한 구체적인 지식을 직접 가지고 있습니다. 이는 로더(Loader)와 변환(Transformation) 로직 사이의 강한 결합(Tight Coupling)을 유발하며, "관심사 분리" 원칙에 어긋납니다. 또한, `BuildTarget`이 `src/core/model.rs`에 위치하여 데이터 모델들과 섞여 있는 상태입니다.

## 2. 제안된 아키텍처

### 2.1 `BuildTarget` 추출
`BuildTarget`을 `src/core/model.rs`에서 새로운 모듈인 `src/core/target.rs`로 이동합니다. 이를 통해 타겟 관련 로직과 상수를 위한 전용 공간을 확보합니다.

### 2.2 메타데이터 처리 로직 캡슐화
`ResourceParser`에 있던 메타데이터 병합 및 정화(Cleanup) 로직을 `BuildTarget` 내부로 이동합니다. 이는 "정보 전문가(Information Expert)" 원칙에 따라, 데이터를 가진 객체가 해당 데이터를 처리하는 로직도 가지게 함을 의미합니다.

### 2.3 `ResourceParser` 결합도 낮추기
`ResourceParser`는 구체적인 키 이름이나 타겟별 구현 세부 사항을 알 필요 없이, `BuildTarget`이 제공하는 추상화된 메서드를 호출하여 메타데이터를 처리합니다.

## 3. 수정된 컴포넌트 모델

### `src/core/target.rs` (신규)
```rust
pub enum BuildTarget {
    GeminiCli,
    ClaudeCode,
    OpenCode,
}

impl BuildTarget {
    pub fn reserved_key(&self) -> &'static str { ... }
    pub fn all_reserved_keys() -> &'static [&'static str] { ... }
    pub fn merge_metadata(&self, base: &mut Value, external: &Value) { ... }
}
```

### `src/loader/parser.rs` (리팩토링 후)
```rust
impl ResourceParser {
    fn merge_metadata(&self, base: &mut Value, external: &Value) {
        // 이제 타겟에게 처리를 위임함
        self.target.merge_metadata(base, external);
    }
}
```

## 4. 기대 효과
- **확장성**: 새로운 타겟 추가 시 `BuildTarget`만 수정하면 됨.
- **유지보수성**: 타겟 관련 로직이 한 곳으로 집중됨.
- **테스트 용이성**: 메타데이터 병합 로직을 `BuildTarget` 단위에서 독립적으로 테스트 가능.
