# TASK: Integrate MetadataMerger into ResourceParser

## 개요
`ResourceParser`가 메타데이터 병합을 직접 수행하지 않고 `MetadataMerger`를 통해 처리하도록 수정합니다.

## 작업 상세

### 1. `ResourceParser` 수정 (`src/loader/parser.rs`)
- 생성 시 `MetadataMap`을 주입받거나 `parse_resource` 시 전달받도록 변경합니다.
- `merge_metadata` 메서드를 제거하고, 대신 `MetadataMerger`를 인스턴스화하여 사용합니다.

```rust
// 예시
let merger = MetadataMerger::new(self.target, self.map.as_ref());
let final_metadata = merger.merge(&fm_metadata, ext_metadata.as_ref())?;
```

## 검증 방법
- 기존의 모든 리소스 로딩 테스트를 실행하여 기능 저하가 없는지 확인합니다.
