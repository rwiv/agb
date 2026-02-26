# Implementation Plan: Type-Aware Registry & Core Relocation

## 1. 개요 (Overview)
`Registry`를 `core` 모듈로 이동하고 리소스의 타입 정보를 기반으로 중복을 체크하도록 개선합니다.

## 2. 작업 목표 (Goals)
- [ ] `src/core/model.rs` 내 `ResourceType` 및 `Resource::r_type()` 구현.
- [ ] `src/builder/registry.rs`를 `src/core/registry.rs`로 이동.
- [ ] `Registry`의 내부 구조를 타입과 이름을 모두 고려하도록 수정.
- [ ] 관련 파일들의 임포트 경로 업데이트.
- [ ] 신규 중복 체크 로직(타입 다를 시 이름 중복 허용) 검증을 위한 테스트 케이스 추가.

## 3. 작업 단계 (Phases)

### Phase 1: Core 모델 및 구조 변경
- [ ] **TASK-1-1**: `src/core/model.rs` 수정 (`ResourceType` 추가 및 `Resource` 확장)
- [ ] **TASK-1-2**: `Registry` 파일 이동 및 `src/core/mod.rs` 업데이트

### Phase 2: Registry 기능 고도화
- [ ] **TASK-2-1**: `Registry` 내부 `HashMap` 키 및 `register` 로직 수정
- [ ] **TASK-2-2**: `Registry` 내의 테스트 코드 업데이트 (타입 기반 중복 체크 검증)

### Phase 3: 전역 임포트 정리 및 최종 검증
- [ ] **TASK-3-1**: `src/builder/mod.rs` 및 관련 파일의 임포트 경로 수정
- [ ] **TASK-3-2**: 전체 빌드 및 테스트 실행 (`cargo test`)

## 4. 의존성 (Dependencies)
- `Phase 1` 작업은 다른 모든 작업의 기초가 됨.
- `Registry` 구조 변경 후 반드시 테스트 케이스를 최신화해야 함.
