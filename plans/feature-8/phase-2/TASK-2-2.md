# TASK-2-2: Registry 테스트 코드 업데이트

## 1. 작업 개요 (Summary)
`src/core/registry.rs` 내의 테스트 코드를 업데이트하여 동일한 이름이라도 타입이 다를 때 중복이 발생하지 않는지 검증합니다.

## 2. 세부 구현 사항 (Implementation Details)

### 2.1. 테스트 케이스 추가
- `test_register_same_name_different_type()` 추가.
- `write-plan`이라는 이름으로 `Command`와 `Skill`이 동시에 등록되는 상황 시뮬레이션.
- 두 리소스 모두 `register()` 성공 확인.

### 2.2. 기존 테스트 케이스 수정
- `test_register_and_conflict()`에서 동일 타입, 동일 이름의 경우 여전히 충돌이 발생하는지 확인.

## 3. 검증 방법 (Verification)
- `cargo test --lib core::registry`를 실행하여 모든 테스트 패스 확인.
