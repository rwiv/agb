# TASK-2-2: ResourceParser 수정

## 목적
`ScannedResource` 정보를 바탕으로 최종 `Resource` 객체를 조립합니다.

## 작업 내용
- `src/loader/parser.rs` 수정
    - `parse_resource` 메서드가 `ScannedResource`를 인자로 받도록 변경.
    - `ScannedPaths::Skill`의 `extras` 경로들을 `ExtraFile` 정보(복사 대상)로 변환하여 `SkillData` 생성.

## 검증 계획
- 바이너리 파일 등이 포함된 스킬이 정상적인 `Resource::Skill` 객체로 파싱되는지 확인.
