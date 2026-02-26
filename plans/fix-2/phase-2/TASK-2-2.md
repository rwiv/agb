# TASK-2-2: 통합 검증 (E2E 테스트)

## 목표
실제 파일 시스템 환경에서 멀티라인 `description`이 포함된 리소스의 동기화가 안전하게 거부되는지 확인합니다.

## 작업 내용
1. `tests/e2e_sync_multiline_test.rs` 파일 생성 (또는 기존 E2E 테스트에 추가).
2. 멀티라인 `description`을 가진 임시 소스 파일 생성.
3. `Syncer::sync_resource` 또는 CLI를 통한 동기화 시도.
4. 에러가 발생하고 소스 파일의 내용이 변하지 않았음을 검증.
