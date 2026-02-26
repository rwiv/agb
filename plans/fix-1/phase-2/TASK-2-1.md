# TASK-2-1: Syncer 클래스 중심으로 동기화 흐름 통합

## 목적
여러 모듈에 흩어진 동기화 로직을 `Syncer` 구조체로 일원화하여 유지보수성을 높이고 코드 중복을 제거합니다.

## 작업 내용

1. **Syncer 구조 변경 (`src/syncer/mod.rs`)**
   - `SkillSyncer`에서 수행하던 마크다운 패치 로직을 `Syncer::sync_resource`로 완전히 통합.
   - `Resource` 타입에 따른 분기 로직을 정돈하여 중복된 파일 읽기/쓰기 제거.

2. **SkillSyncer 역할 축소 (`src/syncer/skill.rs`)**
   - 마크다운 본문 처리를 제외한 '디렉터리 내 일반 파일(extras) 동기화' 기능에만 집중하도록 수정.
   - 불필요한 호환성 함수(예: `update_description`, `replace_content` 등)를 제거하거나 `MdPatcher`를 직접 활용하도록 변경.

## 검증 방법
- 전체 리소스 타입(Command, Agent, Skill)에 대해 `agb sync` 명령을 실행하여 동일하게 동작하는지 확인.
- `cargo test`를 통해 기존 통합 테스트 통과 확인.
