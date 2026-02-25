# TASK-2-1: 테스트 픽스처 변환 (.json -> .yaml)

## 개요
`tests/fixtures` 디렉터리에 존재하는 JSON 파일들을 YAML로 변환하여 시스템이 YAML만으로 동작하도록 합니다.

## 작업 내용
- `tests/fixtures/commands/foo.json` -> `foo.yaml` (또는 삭제 후 필요시 yaml 생성)
- `tests/fixtures/plugins/plugin_a/commands/foo.json` -> `foo.yaml`
- `tests/fixtures/plugins/plugin_b/commands/foo.json` -> `foo.yaml`
- `tests/fixtures/skills/python_expert/python_expert.json` -> `SKILL.yaml` (이전 커밋에서 이미 수정되었을 수 있으니 확인)

## 검증 방법
- 파일 시스템 확인 및 `cargo test` 실행 시 픽스처 로딩 에러 여부 확인
