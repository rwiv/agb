# TASK-3-1: src/builder/core.rs 내 로더 및 팩토리 호출부 수정

## 목표
- 빌드 오케스트레이터인 `Builder`가 새로 구현된 객체 기반 로더와 팩토리를 사용하도록 수정.

## 상세 작업 내용

### 1. `Builder::run` 메서드 수정
- 기존 함수 호출 방식을 구조체 인스턴스 생성 및 메서드 호출 방식으로 변경.
  ```rust
  // AS-IS
  let files = loader::scan_plugins(...)?;
  let resources = loader::load_resources(...)?;
  let transformer = transformer::get_transformer(...);

  // TO-BE
  let loader = ResourceLoader::new(root, exclude)?;
  let resources = loader.load()?;
  let transformer = TransformerFactory::create(&cfg.target);
  ```

## 검증 계획
- 빌드가 정상적으로 컴파일되는지 확인.
- `cargo run`을 통해 실제 예제 프로젝트 빌드 확인.
