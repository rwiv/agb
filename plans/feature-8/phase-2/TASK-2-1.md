# TASK-2-1: Registry 내부 로직 및 데이터 구조 수정

## 1. 작업 개요 (Summary)
`Registry` 내부의 `HashMap` 키를 `(ResourceType, String)`으로 변경하고, `register()` 메서드에서 이를 활용하여 중복을 체크하도록 수정합니다.

## 2. 세부 구현 사항 (Implementation Details)

### 2.1. HashMap 키 변경
- `Registry` 구조체 내의 `resources: HashMap<String, Resource>`를 `resources: HashMap<(ResourceType, String), Resource>`로 수정.

### 2.2. register() 메서드 수정
- `register()` 내에서 `(resource.r_type(), resource.name().to_string())`을 키로 사용하여 중복 체크 수행.
- 에러 메시지에 리소스 타입을 포함하도록 업데이트.

## 3. 검증 방법 (Verification)
- `cargo check` 및 `cargo test` 실행.
- 기존 테스트 코드에서의 컴파일 에러가 발생하는지 확인 (TASK-2-2에서 수정 예정).
