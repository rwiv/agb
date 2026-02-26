# 태스크 2-2: 메타데이터 병합 로직 이동

## 1. 목표
`src/loader/parser.rs`에 있는 `merge_metadata` 로직을 `src/core/target.rs`의 `BuildTarget` 구현체로 완전히 이전합니다.

## 2. 작업 내용
- `parser.rs`의 `merge_metadata` 메서드 내부 구현 복사.
- `target.rs`의 `BuildTarget::merge_metadata`에 붙여넣기 및 `self`를 활용하도록 수정.
- `serde_json` 및 `Value` 타입 관련 의존성 처리.

## 3. 성공 기준
- `BuildTarget`이 독립적으로 메타데이터 병합 및 정화(Cleanup) 기능을 수행할 수 있음.
- `target.rs` 내에 해당 로직에 대한 단위 테스트가 작성됨.
