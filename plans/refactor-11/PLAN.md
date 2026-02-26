# PLAN: Skill 동기화 로직 리팩토링 및 중복 제거

## 개요
스킬 리소스 동기화 시 발생하는 `SKILL.md` 중복 패치 로직을 제거하고, `Syncer`와 `SkillSyncer` 간의 책임을 명확히 분리합니다.

## 단계별 계획

### Phase 1: SyncPlanner 정제
- [ ] `src/syncer/planner.rs`: `SyncAction::PatchMarkdown` 제거
- [ ] `src/syncer/planner.rs`: `SKILL.md`를 분석 대상에서 제외
- [ ] `src/syncer/planner.rs`: 관련 테스트 코드 수정

### Phase 2: SkillSyncer 리팩토링
- [ ] `src/syncer/skill.rs`: `SyncAction::PatchMarkdown` 핸들링 로직 제거
- [ ] `src/syncer/skill.rs`: `SkillSyncer` 구조체와 독립 함수 간의 중복 정리 (하나의 엔트리포인트로 통일)
- [ ] `src/syncer/skill.rs`: 불필요한 `Transformer`, `ResourceType` 임포트 제거

### Phase 3: Syncer 연결 및 검증
- [ ] `src/syncer/mod.rs`: 변경된 `SkillSyncer` 호출부 확인 및 수정
- [ ] 전체 동기화 테스트 (`tests/e2e_sync_test.rs` 등) 수행하여 회귀 오류 확인

## 성공 기준
1. `SKILL.md` 파일은 `Syncer::sync_resource`에 의해 단 한 번만 패치되어야 함.
2. `SkillSyncer`는 스킬 디렉토리 내의 추가 파일들(extras)에 대해서만 Add/Update/Delete를 수행해야 함.
3. 기존 E2E 동기화 테스트가 모두 통과해야 함.
