# TASK 4: 기존 멀티라인 실패 테스트를 성공 케이스로 전환

## 상세 설명
현재 `tests/e2e_sync_multiline_test.rs`에 있는 `test_sync_fails_on_multiline_description` 테스트는 멀티라인 감지 시 에러가 발생하는 것을 기대합니다. 이 테스트를 멀티라인 동기화가 성공하는 것을 확인하는 테스트로 변경합니다.

## 수락 기준
- `tests/e2e_sync_multiline_test.rs`의 테스트 이름 및 로직을 성공 확인용으로 변경.
- 타겟에서 수정된 멀티라인 `description`이 `agb sync` 수행 후 소스 파일에 정확히 반영되는지 확인.
- 동기화 후 소스 파일의 다른 메타데이터와 본문이 훼손되지 않는지 확인.
