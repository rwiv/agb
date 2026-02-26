# TASK-2-2: skill.rs 내 중복 함수 제거 및 SkillSyncer 정제

## 목적
`src/syncer/skill.rs`에 존재하는 중복된 헬퍼 함수들을 제거하고, `SkillSyncer`가 `MdPatcher`를 직접 활용하도록 구조를 개선하여 코드 응집도를 높입니다.

## 작업 내용

1. **중복 함수 제거 (`src/syncer/skill.rs`)**
   - `update_description`, `diff_content`, `replace_content` 등 `MdPatcher`를 단순 래핑하고 있는 함수들을 제거.
   - 해당 함수들을 참조하던 코드를 `MdPatcher` 직접 호출 방식으로 변경.

2. **SkillSyncer 정제**
   - `SkillSyncer::sync_skill_dir` 내의 마크다운 패치 로직이 `Syncer` 모듈의 공통 로직을 사용하도록 분리하거나, `MdPatcher` 사용 방식을 일원화.
   - `SkillSyncer` 구조체와 독립 함수(`sync_skill_dir`) 간의 혼선을 정리하여 하나의 명확한 엔트리포인트만 유지.

## 검증 방법
- 컴파일 에러 여부 확인.
- `cargo test`를 실행하여 기존 테스트 케이스들이 정상적으로 작동하는지 확인.
