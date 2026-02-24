# TASK-1-2: 리소스 파싱 로직 추출

## 1. 개요
그룹화된 파일 경로로부터 실제 데이터를 읽고 파싱하여 `Resource` 객체를 생성하는 로직을 추출합니다.

## 2. 작업 내용
- `src/resource/loader.rs`에 `parse_resource` 함수 정의
- 기존 148-183행의 로직(파일 읽기, JSON/YAML 파싱, `ResourceData` 생성)을 이동
- 개별 리소스 하나에 대한 처리를 담당하게 함

```rust
fn parse_resource(
    key: ResourceKey,
    paths: ResourcePaths
) -> Result<Resource> {
    // 1. md_path 읽기
    // 2. metadata_path 읽기 및 파싱
    // 3. Resource Enum 타입 결정 및 반환
}
```

## 3. 검증 계획
- 잘못된 형식의 JSON/YAML 파일에 대한 에러 처리가 올바르게 동작하는지 확인
