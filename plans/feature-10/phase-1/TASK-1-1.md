# TASK 1-1: Add `source_path` to `ResourceData`

## 개요 (Description)
`Syncer`가 소스 파일을 찾아 수정하거나 삭제할 수 있도록 `core::model::ResourceData`에 원본 소스의 절대 경로를 저장하는 필드를 추가합니다.

## 수정 파일 (Files to Modify)
- `src/core/model.rs`

## 상세 지침 (Actionable Instructions)
1. `ResourceData` 구조체에 `source_path: std::path::PathBuf` 필드를 추가합니다.
2. `ResourceData`의 `Clone`, `Serialize`, `Deserialize` 파생(Derive)이 유지되는지 확인합니다.
3. 리소스 타입별로 `source_path`가 의미하는 바를 주석으로 명시합니다.
    - `Command`, `Agent`: 마크다운(`.md`) 파일의 절대 경로.
    - `Skill`: 스킬 디렉터리의 절대 경로.
4. `Registry`의 테스트 코드(`mock_resource`) 등 `ResourceData`를 생성하는 모든 위치에서 임의의 경로를 전달하도록 수정합니다.

## 검증 방법 (Verification)
- `cargo check`를 실행하여 컴파일 에러가 없는지 확인합니다.
- `src/core/model.rs`의 유닛 테스트를 실행합니다.
