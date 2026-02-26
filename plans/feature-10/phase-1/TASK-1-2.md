# TASK 1-2: Update `ResourceParser` to Save Source Paths

## 개요 (Description)
`loader::parser::ResourceParser`에서 리소스를 파싱하고 객체로 조립하는 시점에, 스캔된 파일의 원본 절대 경로를 `ResourceData`의 `source_path` 필드에 저장하도록 수정합니다.

## 수정 파일 (Files to Modify)
- `src/loader/parser.rs`

## 상세 지침 (Actionable Instructions)
1. `ResourceParser::parse_resource` 메서드에서 `ScannedResource`로부터 각 리소스의 `source_path`를 결정합니다.
    - `Command`, `Agent`: `md` 파일의 절대 경로 (항상 존재해야 함).
    - `Skill`: `SKILL.md`가 포함된 디렉터리의 절대 경로 (부모 경로 추출).
2. `parse_common` 메서드가 `source_path`를 인자로 받도록 수정하고, 반환되는 `ResourceData` 객체의 `source_path` 필드를 설정합니다.
3. `Skill` 타입 리소스 생성 시 `SkillData`의 `base.source_path`에 스킬 디렉터리 경로가 저장되는지 확인합니다.

## 검증 방법 (Verification)
- `cargo test loader::parser::tests`를 실행하여 경로가 올바르게 저장되는지 확인하는 새 테스트 케이스를 추가하거나 기존 테스트를 업데이트합니다.
- `SKILL.md`의 부모 디렉터리가 정확히 스킬 루트로 인식되는지 확인합니다.
