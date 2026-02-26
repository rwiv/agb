# TASK-4-1: Skill 다중 파일 빌드 통합 테스트

## 목적
실제 파일 시스템 시나리오를 통해 Skill의 추가 파일들이 올바르게 복사되는지 검증합니다.

## 작업 내용
- `tests/fixtures/` 내에 다중 파일을 포함하는 테스트용 Skill 추가:
    - `plugins/test_plugin/skills/multi_file/SKILL.md`
    - `plugins/test_plugin/skills/multi_file/data/config.json`
    - `plugins/test_plugin/skills/multi_file/scripts/run.py`
- `tests/e2e_build_test.rs`에 새로운 테스트 케이스 추가:
    - 빌드 후 `target/skills/multi_file/data/config.json`이 존재하는지 확인.
    - 빌드 후 `target/skills/multi_file/scripts/run.py`가 존재하는지 확인.
    - 파일 내용이 원본과 일치하는지 확인.

## 검증 계획
- `cargo test`를 실행하여 통합 테스트 통과 확인.
