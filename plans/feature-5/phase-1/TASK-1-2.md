# Task 1.2: 기존 Claude/OpenCode 로직 이관 및 테스트

## 개요
기존에 작성된 `claude.rs`와 `opencode.rs`의 유닛 테스트를 `DefaultTransformer` 기반으로 재작성하여 기능적 동등성을 확인합니다.

## 상세 작업 내용

1.  **유닛 테스트 이관**:
    - `src/transformer/default.rs`에 Claude와 OpenCode 시나리오를 검증하는 테스트 코드 추가.
2.  **호환성 확인**:
    - 기존에 생성되던 파일 경로와 마크다운 구조가 `DefaultTransformer`에서도 동일하게(또는 의도한 대로 `metadata` 중심으로) 생성되는지 확인.

## 성공 기준
- `default.rs` 내의 모든 테스트가 통과한다.
- `metadata` 중심의 구조가 기존 Claude의 `description`/`parameters` 분리 방식보다 더 유연함을 테스트로 입증한다.
