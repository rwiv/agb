# PLAN: Skill Extra Files Inclusion & Refactoring

## Phase 1: 모델 재구조화
- [x] **TASK-1-1**: `src/core/model.rs` 정리 및 `ExtraFile`, `TransformedResource` 등 도메인 모델 정의
- [x] **TASK-1-2**: `src/loader/` 내부에 `ScannedResource` 등 로더 전용 모델 정의 (기존 `ResourceKey/Paths` 대체)

## Phase 2: 리소스 로더 수정
- [x] **TASK-2-1**: `src/loader/resolver.rs` 수정 (신규 모델 사용 및 추가 파일 경로 수집)
- [x] **TASK-2-2**: `src/loader/parser.rs` 수정 (신규 모델 기반 `Resource` 생성 로직 구현)

## Phase 3: 빌더 및 에미터 수정
- [x] **TASK-3-1**: `src/builder/mod.rs` 수정 (`TransformedResource` 그룹화 로직)
- [x] **TASK-3-2**: `src/builder/emitter.rs` 수정 (시그니처 변경 및 복사 로직 추가)

## Phase 4: 검증 및 테스트
- [x] **TASK-4-1**: Skill 다중 파일 빌드 통합 테스트 (E2E)
- [x] **TASK-4-2**: 전체 빌드 사이클 및 Clean 동작 검증
