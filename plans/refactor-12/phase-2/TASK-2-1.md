# TASK 2-1: DirectorySyncer 도입 (SkillSyncer 일반화)

## 목적
스킬 디렉토리 동기화 로직을 범용적인 `DirectorySyncer`로 리팩토링하여, 확장성을 높이고 스킬 의존성을 줄입니다.

## 작업 내용
- `src/syncer/skill.rs`를 `src/syncer/directory.rs`로 이동 및 리네임.
- `SkillSyncer` 구조체와 연관 함수들을 `DirectorySyncer`로 리네임.
- `src/syncer/mod.rs`에서 `skill::SkillSyncer::sync_skill_dir` 호출부를 `DirectorySyncer::sync` (또는 유사 이름) 호출로 변경.
- 관련 모듈 가시성(visibility) 및 임포트(`mod.rs`) 수정.

## 검증 방법
- `cargo check`를 통해 컴파일 오류 확인.
- `tests/e2e_skill_extras_test.rs` 등 관련 테스트 수행.
