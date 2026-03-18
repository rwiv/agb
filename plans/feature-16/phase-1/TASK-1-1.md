# Task 1.1: `Cargo.toml` 의존성 업데이트

## 개요
기존 `serde_json`은 객체 파싱 시 기본적으로 알파벳 순으로 키를 정렬하는 `BTreeMap`을 사용하여 프론트매터 원본의 키 순서가 유실되는 문제가 있었습니다. 
이를 해결하기 위해 `serde_json`의 `preserve_order` 피처를 활성화하여, 내부 자료구조가 삽입 순서를 기억하는 `IndexMap`을 사용하도록 변경합니다.

## 작업 내용
1. 프로젝트 루트의 `Cargo.toml` 파일을 연다.
2. `[dependencies]` 섹션 내의 `serde_json` 선언을 찾는다.
3. 기존 선언(`serde_json = "1.0"`)을 `serde_json = { version = "1.0", features = ["preserve_order"] }` 형태로 수정한다.
4. `cargo build` 명령어를 실행하여 전체 프로젝트가 에러 없이 컴파일되는지 확인한다.

## 갱신 대상 파일
- `Cargo.toml`

## 성공 기준
* `Cargo.toml`의 `serde_json` 선언에 `features = ["preserve_order"]`가 명시되어 있어야 한다.
* 프로젝트 전체 컴파일이 오류 없이 성공해야 한다.
