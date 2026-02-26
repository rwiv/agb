# PLAN: Syncer 모듈 버그 수정 및 안정화 계획

## Phase 1: 기반 구조 개선 및 버그 수정 (Fixing Core Issues)
가장 시급한 마크다운 파괴 버그와 비효율적인 경로 계산 로직을 수정합니다.

- [ ] **TASK-1-1**: `Transformer` 인터페이스 개선 및 경로 계산 최적화
- [ ] **TASK-1-2**: `MdPatcher` 정규표현식 강화 및 멀티라인 지원
- [ ] **TASK-1-3**: `SkillSyncer`의 마크다운 중첩 삽입 버그 수정

## Phase 2: 리팩터링 및 책임 분리 (Refactoring)
코드 중복을 제거하고 모듈 간의 역할을 명확히 정의합니다.

- [ ] **TASK-2-1**: `Syncer` 클래스 중심으로 동기화 흐름 통합
- [ ] **TASK-2-2**: `skill.rs` 내 중복 함수 제거 및 `SkillSyncer` 정제
- [ ] **TASK-2-3**: 에러 핸들링 강화 및 `unwrap()` 제거

## Phase 3: 검증 및 테스트 (Validation)
수정된 로직이 기존 기능을 파괴하지 않는지 확인하고, 새로운 테스트 케이스를 추가합니다.

- [ ] **TASK-3-1**: 마크다운 보존(Formatting Preservation) 단위 테스트 추가
- [ ] **TASK-3-2**: 다양한 타겟(Gemini, Claude)에서의 통합 동기화 테스트 수행
- [ ] **TASK-3-3**: 버그 리포트 기반의 엣지 케이스(Edge Cases) 검증
