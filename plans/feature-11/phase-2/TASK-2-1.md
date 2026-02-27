# TASK: Create MetadataMerger & Migrate Logic

## 개요
기존 `ResourceParser`에 포함되어 있던 메타데이터 병합 로직을 별도의 `MetadataMerger` 모듈로 분리합니다.

## 작업 상세

### 1. `src/loader/merger.rs` 생성
- `MetadataMerger` 구조체와 기본적인 병합 메서드를 정의합니다.

```rust
use crate::core::{BuildTarget, MetadataMap};
use serde_json::{Value, json};
use anyhow::Result;

pub struct MetadataMerger<'a> {
    target: BuildTarget,
    map: Option<&'a MetadataMap>,
}

impl<'a> MetadataMerger<'a> {
    pub fn new(target: BuildTarget, map: Option<&'a MetadataMap>) -> Self {
        Self { target, map }
    }

    /// Frontmatter(base)와 외부 YAML(external)을 병합합니다.
    pub fn merge(&self, base: &Value, external: Option<&Value>) -> Result<Value> {
        let mut merged = base.clone();
        
        // TODO: TASK-2-2에서 Mapping 적용 로직 추가
        
        if let Some(ext) = external {
            self.apply_external_override(&mut merged, ext)?;
        }

        self.cleanup(&mut merged);
        Ok(merged)
    }
}
```

### 2. 로직 이관
- `src/loader/parser.rs`의 `merge_metadata`와 `apply_external_override`(또는 유사한 이름의 내부 로직)을 `merger.rs`로 옮깁니다.
- 타겟 전용 예약어 섹션 처리 및 클린업 로직을 포함합니다.

## 검증 방법
- 기존 `parser.rs`에 있던 테스트 케이스를 `merger.rs`로 옮겨서 동일하게 통과하는지 확인합니다.
