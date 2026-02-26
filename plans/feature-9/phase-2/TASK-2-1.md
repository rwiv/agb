# TASK-2-1: ResourcePathResolver 수정

## 목적
기존의 `ResourceKey/Paths` 구조를 버리고 `ScannedResource` 모델을 사용하여 파일을 그룹화합니다.

## 작업 내용
- `src/loader/resolver.rs` 수정
    - `resolve` 메서드가 `Vec<ScannedResource>`를 반환하도록 변경.
    - `Skill` 타입 스캔 시 `SKILL.md` 등을 제외한 파일들을 `ScannedPaths::Skill.extras`에 수집.

## 검증 계획
- 유닛 테스트를 통해 `ScannedResource` 리스트가 정확하게 생성되는지 확인.
