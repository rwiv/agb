# TASK-1-2: Loader 내부 모델 정의

## 목적
`core`에서 삭제된 모델들을 대체하고, 파일 스캔 정보를 효율적으로 관리할 로더 전용 모델을 구축합니다.

## 작업 내용
- `src/loader/mod.rs` 또는 신규 파일에 정의:
    - `ScannedResource` 구조체: `plugin`, `name`, `paths` 포함.
    - `ScannedPaths` Enum: 리소스 타입별로 경로 구성을 강제 (`Skill`만 `extras` 필드 포함).

## 검증 계획
- `loader` 모듈 내에서 새로운 모델이 정상적으로 참조되는지 확인.
