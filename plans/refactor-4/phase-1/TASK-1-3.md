# TASK-1-3: ResourcePathResolver 구현 (경로 분석 로직 분리)

## 목표
- 파일 경로를 분석하여 리소스의 종류(Command, Agent, Skill)와 구성 정보를 추출하는 `ResourcePathResolver` 구현.
- `skills` 디렉터리의 특수 구조와 일반 파일 구조의 매핑 로직을 명확히 분리.

## 상세 작업 내용

### 1. `ResourcePathResolver` 구현 (`src/resource/loader/resolver.rs`)
- 인터페이스 설계:
  ```rust
  pub struct ResourcePathResolver;

  impl ResourcePathResolver {
      pub fn resolve(&self, root: &Path, files: &[PathBuf]) -> Result<HashMap<ResourceKey, ResourcePaths>>;
  }
  ```
- 기존 `group_files_by_resource` 로직을 이 객체로 이동.
- 내부적으로 `Skill`과 `Command/Agent` 경로 분석 로직을 프라이빗 메서드로 분리하여 가독성 향상.

### 2. 데이터 구조 최적화
- `Option` 튜플 대신 의미 있는 내부 구조체 사용 고려 (예: `RawResourceGroup`).

## 검증 계획
- `ResourcePathResolver` 유닛 테스트 구현 (`src/resource/loader/resolver.rs` 내):
  - 기존 `loader.rs`의 `test_group_files_by_resource_logic` 로직을 이동 및 리팩토링.
  - 중복된 메타데이터 포맷(`foo.json`, `foo.yaml`) 존재 시 에러 발생 확인 (기존 `test_metadata_conflict_error` 이동).
  - 다양한 경로 조합에 대한 식별 테스트:
    - `plugin/commands/foo.md` -> Command "foo"
    - `plugin/skills/task/task.yaml` -> Skill "task" 메타데이터
    - 비정상적인 깊이의 경로 무시 확인.
