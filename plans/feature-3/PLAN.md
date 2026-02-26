# Plan: Path Expansion & YAML Metadata

## 개요
`agb`의 사용성을 높이기 위해 홈 디렉터리 경로 확장 기능과 YAML 메타데이터 지원 기능을 구현합니다.

## Phase 1: 환경 설정 및 경로 확장
- [ ] TASK-1-1: 의존성 추가 및 기본 환경 설정 (`shellexpand`) @plans/feature-2/phase-1/TASK-1-1.md
- [ ] TASK-1-2: `source` 필드 경로 확장 구현 (`config.rs`) @plans/feature-2/phase-1/TASK-1-2.md

## Phase 2: YAML 메타데이터 및 하이브리드 지원
- [ ] TASK-2-1: YAML 확장자 스캔 지원 (`loader.rs`) @plans/feature-2/phase-2/TASK-2-1.md
- [ ] TASK-2-2: 확장자별 메타데이터 파싱 및 충돌 감지 로직 구현 @plans/feature-2/phase-2/TASK-2-2.md

## Phase 3: 검증 및 테스트
- [ ] TASK-3-1: 통합 테스트 및 예외 케이스 검증 @plans/feature-2/phase-3/TASK-3-1.md
