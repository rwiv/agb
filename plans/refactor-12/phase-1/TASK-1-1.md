# TASK 1-1: Resource 모델 다형성 강화 (Helper Methods 추가)

## 목적
`Resource` Enum에 헬퍼 메서드를 추가하여 `Syncer::sync_resource`의 복잡도를 낮추고, 리소스 타입에 관계없이 일관된 방식으로 데이터를 조회할 수 있도록 합니다.

## 작업 내용
- `src/core/model.rs` 파일 수정:
    - `Resource` Enum에 `main_source_path() -> PathBuf` 메서드 추가. (Command/Agent는 `source_path`, Skill은 `base.source_path.join(SKILL_MD)` 반환)
    - `Resource` Enum에 `metadata() -> &Value` 메서드 추가.
    - `Resource` Enum에 `content() -> &str` 메서드 추가.
- `src/syncer/mod.rs` 파일 수정:
    - `sync_resource` 함수 내에서 `match resource` 분기 로직을 헬퍼 메서드 사용으로 교체.
    - `source_file_content` 로딩부와 `write_path` 결정부를 간소화.

## 검증 방법
- `cargo test` 실행하여 기존 유닛 테스트 및 E2E 테스트 통과 확인.
