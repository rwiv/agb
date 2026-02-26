# Task 3.1: 기존 테스트 및 의존성 업데이트

## 1. 개요 (Overview)
`src/lib.rs` 도입 및 `app` 모듈 리팩토링 이후, 기존의 모든 테스트(유닛 테스트, E2E 테스트)가 정상적으로 동작하는지 확인하고 필요한 의존성 참조를 업데이트합니다.

## 2. 작업 상세 (Implementation Details)

### 2.1. E2E 테스트 확인 (`tests/`)
- `tests/e2e_build_test.rs`, `tests/e2e_sync_test.rs`, `tests/cli_test.rs` 등을 실행합니다.
- 이 테스트들은 `assert_cmd`를 사용하여 바이너리를 직접 실행하므로, CLI 인터페이스(`agb build`, `agb sync`)가 유지되었다면 별도의 코드 수정 없이 통과해야 합니다.

### 2.2. 유닛 테스트 경로 업데이트
- 각 모듈(`src/loader`, `src/builder` 등) 내부의 `#[cfg(test)]` 블록에서 `crate::` 참조가 올바른지 확인합니다.
- 리팩토링 과정에서 `pub`으로 변경된 가시성 범위를 체크하고, 테스트 코드에서 접근 가능한지 확인합니다.

## 3. 검증 방법 (Verification)
- `cargo test` 명령을 실행하여 모든 테스트 케이스가 통과하는지 확인합니다.
