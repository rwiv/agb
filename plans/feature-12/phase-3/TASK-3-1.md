# TASK 3-1: Integrate with `Builder::run`

## 개요 (Description)
빌드 프로세스 시작 시 실제 변환 작업이 수행되기 전에 의존성 검사를 실행하도록 통합합니다.

## 수정 파일 (Files to Modify)
- `src/builder/mod.rs`

## 상세 지침 (Actionable Instructions)
1. `Builder::run` 메서드 초기에 `DependencyChecker` 인스턴스를 생성합니다.
2. `checker.check_dependencies(registry, source_dir)?`를 호출하여 검사를 수행합니다.
3. `src/builder/mod.rs` 상단에 `mod dependency;`를 추가하여 모듈을 등록합니다.

## 검증 방법 (Verification)
- `cargo check`를 실행하여 컴파일 에러가 없는지 확인합니다.
