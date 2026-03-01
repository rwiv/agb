# TASK-1-1: ResourceLoader 기반 구조 및 FileFilter 구현

## 목표
- `src/resource/loader/` 디렉터리 생성 및 모듈 구조 정의.
- `FileFilter` 구조체를 구현하여 스캔 시 제외 패턴 및 금지된 파일 체크 로직을 캡슐화.

## 상세 작업 내용

### 1. 디렉터리 및 모듈 파일 생성
- `src/resource/loader/mod.rs` 생성: `ResourceLoader` 구조체 정의.
- `src/resource/loader/filter.rs` 생성: `FileFilter` 구조체 정의.

### 2. `FileFilter` 구현 (`filter.rs`)
- `exclude_patterns`를 컴파일된 `glob::Pattern`으로 유지.
- 금지된 파일(`GEMINI.md`, `CLAUDE.md`, `AGENTS.md`) 및 숨김 파일 체크 로직 포함.
- 인터페이스 설계:
  ```rust
  pub struct FileFilter {
      patterns: Vec<glob::Pattern>,
  }

  impl FileFilter {
      pub fn new(exclude_patterns: &[String]) -> Result<Self>;
      pub fn is_valid(&self, root: &Path, path: &Path) -> Result<bool>;
  }
  ```

### 3. `ResourceLoader` 기본 구조 정의 (`mod.rs`)
- `ResourceLoader`가 `root` 경로와 `FileFilter`를 보유하도록 설계.
- 기존 `scan_plugins` 로직을 `loader.scan()` 메서드로 이동.

## 검증 계획
- `FileFilter` 유닛 테스트 구현 (`src/resource/loader/filter.rs` 내):
  - 기존 `loader.rs`의 `test_forbidden_files_in_plugins` 로직을 이동 및 `FileFilter`에 맞게 리팩토링.
  - 지정된 패턴(`*.tmp`)이 올바르게 필터링되는지 확인.
  - 숨김 파일(`.git` 등)이 필터링되는지 확인.
  - 유효하지 않은 Glob 패턴 입력 시 에러 처리 확인.
