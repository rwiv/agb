# Task 3.1: 기존 트랜스포머 파일 제거 및 모듈 업데이트

## 개요
불필요해진 `claude.rs`와 `opencode.rs`를 삭제하고 시스템 전체의 모듈 연결을 정리합니다.

## 상세 작업 내용

1.  **파일 삭제**:
    - `src/transformer/claude.rs` 삭제.
    - `src/transformer/opencode.rs` 삭제.
2.  **`src/transformer/mod.rs` 수정**:
    - `claude`, `opencode` 모듈 선언 제거.
    - `default` 모듈 추가.
    - `TransformerFactory` (또는 유사한 팩토리 로직)에서 `Target`에 따라 `DefaultTransformer`를 반환하도록 수정.

## 성공 기준
- 컴파일 에러 없이 빌드가 성공한다.
- 불필요한 코드가 제거되어 코드베이스가 슬림해진다.
