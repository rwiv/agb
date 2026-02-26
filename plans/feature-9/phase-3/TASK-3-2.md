# TASK-3-2: Emitter 시그니처 및 로직 변경

## 목적
구조화된 `TransformedResource`를 처리하여 파일 쓰기와 복사를 수행합니다.

## 작업 내용
- `src/builder/emitter.rs` 수정
    - `emit` 메서드가 `TransformedResource` 목록을 받도록 수정.
    - 각 리소스의 `files`는 `write`, `extras`는 `copy` 수행.

## 검증 계획
- 통합 테스트를 통해 `extras` 파일들이 물리적으로 복사되었는지 확인.
