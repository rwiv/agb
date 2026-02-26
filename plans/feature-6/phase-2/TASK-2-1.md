# Task 2-1: ResourceParser & Loader Modification

## 1. 개요
메타데이터 병합을 위해 `ResourceParser`에 빌드 타겟 정보를 주입하고 관련 구조를 수정합니다.

## 2. 세부 작업
- `ResourceParser` 구조체에 `target: BuildTarget` 필드 추가.
- `ResourceParser::new()`가 `BuildTarget`을 인자로 받도록 수정.
- `ResourceLoader::new()`에서 `BuildTarget`을 받아 `Parser` 생성 시 전달하도록 수정.
- `src/main.rs`에서 `BuildTarget`을 결정하여 `Loader`에 전달하는 흐름 확인.

## 3. 검증 기준
- 기존 컴파일 에러가 없어야 함.
- 모든 리소스 파싱 시점에 `BuildTarget` 정보에 접근 가능해야 함.
