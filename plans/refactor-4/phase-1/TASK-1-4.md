# TASK-1-4: ResourceLoader 최종 통합 및 모듈 업데이트

## 목표
- 분리된 `FileFilter`, `MetadataParser`, `ResourcePathResolver`를 `ResourceLoader`에서 조합하여 전체 로딩 파이프라인 완성.
- `src/resource/mod.rs`를 수정하여 새로운 로더 구조를 외부에 공개.

## 상세 작업 내용

### 1. `ResourceLoader` 메서드 완성 (`src/resource/loader/mod.rs`)
- `load()` 퍼블릭 메서드 구현:
  1. `scan()`: `FileFilter`를 사용하여 파일 목록 추출.
  2. `resolve()`: `ResourcePathResolver`를 사용하여 파일 그룹화.
  3. `parse()`: `MetadataParser`를 사용하여 최종 `Resource` 객체 생성.

### 2. `src/resource/mod.rs` 수정
- 기존 `pub mod loader;`를 유지하되, 내부에서 `ResourceLoader`를 `pub use` 하도록 변경.
- 필요 시 기존 함수 기반 API를 하위 호환성을 위해 유지하거나 과감히 삭제 (빌드 수정 필요).

### 3. 통합 테스트 구성
- `src/resource/loader/mod.rs`에 통합 테스트 구현.
- 기존 `loader.rs`의 `test_scan_and_load_resources`를 `ResourceLoader` 인스턴스를 사용하는 방식으로 변경하여 유지.
- 서브 모듈에서 검증된 로직들이 유기적으로 결합되어 실제 `Resource` 객체들을 올바르게 생성하는지 최종 확인.

## 검증 계획
- `cargo test` 실행 시 모든 하위 모듈(`filter`, `parser`, `resolver`) 및 `mod.rs`의 테스트가 통과하는지 확인.
- 기존에 `src/resource/loader.rs`에서 수행하던 모든 테스트 커버리지가 누락 없이 유지되었는지 확인.
