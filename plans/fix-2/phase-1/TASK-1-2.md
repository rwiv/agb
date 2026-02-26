# TASK-1-2: 멀티라인 감지 시나리오 단위 테스트 추가

## 목표
새로 추가된 검증 로직이 의도한 대로 동작하는지 다양한 케이스로 검증합니다.

## 작업 내용
`src/syncer/patcher.rs`의 `tests` 모듈에 다음 테스트 케이스 추가:
1. `test_update_description_error_on_multiline_input`: `new_desc`에 `
`이 포함된 경우.
2. `test_update_description_error_on_marker_in_source`: 원본에 `|` 마커가 있는 경우.
3. `test_update_description_error_on_indentation_in_source`: 원본에 들여쓰기된 멀티라인 데이터가 있는 경우.
4. `test_update_description_success_on_single_line`: 정상적인 단일 라인 케이스가 여전히 성공하는지 확인.
