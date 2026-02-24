# TASK-1-3: 테스트 코드 및 검증

## 목표
로직 변경에 따라 실패하는 테스트 코드를 수정하고, 통합 테스트 환경(Fixtures)을 업데이트하여 전체 빌드 안정성을 확인합니다.

## 작업 상세

1. **`src/resource/loader.rs` 유닛 테스트 수정**:
   - `test_scan_and_load_resources` 내에서 생성하는 파일명을 `METADATA.json`에서 `my_skill.json`으로 변경.
2. **`tests/fixtures/` 업데이트**:
   - `tests/fixtures/plugins/plugin_c/skills/python_expert/` 내의 `METADATA.json`을 `python_expert.json`으로 변경.
3. **`tests/e2e_build_test.rs` 수정**:
   - 테스트 코드에서 `METADATA.json`을 작성하거나 참조하는 부분을 새로운 명명 규칙에 맞게 수정.
4. **전체 테스트 실행**:
   - `cargo test`를 통해 모든 테스트 통과 확인.

## 검증 계획
- `cargo test` 결과가 모두 `passed`여야 함.
- 실제 빌드 결과물에 메타데이터가 정상적으로 포함되는지 로그 확인.
