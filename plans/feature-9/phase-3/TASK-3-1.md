# TASK-3-1: Builder 수집 로직 수정

## 목적
변환된 파일과 추가 파일들을 하나의 `TransformedResource` 단위로 그룹화합니다.

## 작업 내용
- `src/builder/mod.rs` 수정
    - 빌드 루프 내에서 리소스별로 `TransformedResource` 객체 생성.
    - `files` 필드에는 `transformer.transform()` 결과를 할당.
    - `extras` 필드에는 `resource.extras()` (Skill인 경우)를 할당.
    - 생성된 목록을 `Emitter::emit`으로 전달.

## 검증 계획
- `TransformedResource.extras`에 복사 대상 파일들이 누락 없이 담기는지 확인.
