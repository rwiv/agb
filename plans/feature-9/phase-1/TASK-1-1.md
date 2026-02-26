# TASK-1-1: Core 모델 정리 및 신규 모델 정의

## 목적
`core` 모듈에서 로더 전용 타입을 제거하고, 빌드 결과를 표현하는 순수 도메인 모델들을 정의합니다.

## 작업 내용
- `src/core/model.rs` 수정
    - `ResourceKey`, `ResourcePaths` 삭제.
    - `ExtraFile` 구조체 정의 (`source`, `target`).
    - `TransformedResource` 구조체 정의 (`files`, `extras`).
    - `SkillData` 수정 (`extras: Vec<ExtraFile>`).
    - `Resource::Skill(SkillData)`로 변경.

## 검증 계획
- `cargo check`를 통한 컴파일 에러 발생 확인 (로더 코드에서 대량 발생 예상, 정상적인 과정).
