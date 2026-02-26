# Refactor 3: Transformers 모듈 단수형(transformer)으로 이름 변경

## 1. 개요
현재 `src/transformers` 모듈 이름을 `src/transformer`로 변경하여 프로젝트 내 명명 규칙(builder, resource 등)의 일관성을 확보하고, Rust의 관례적인 단수형 모듈 명명 방식을 따릅니다.

## 2. 주요 작업 내용
- `src/transformers/` 디렉토리를 `src/transformer/`로 이름 변경.
- 모든 소스 코드(`src/main.rs`, `src/builder/core.rs` 등) 내의 모듈 참조 경로 수정.
- 기술 사양서(`SPEC.md`) 및 모든 `README.md` 내의 모듈 경로 정보 업데이트.
- 빌드 및 테스트를 통한 최종 검증.

## 3. 마일스톤 (Milestones)

### Phase 1: 코드 리팩토링 및 모듈 이름 변경
- [ ] TASK 1-1: `src/transformers` 디렉토리 이름을 `src/transformer`로 변경 (git mv 권장)
- [ ] TASK 1-2: 소스 코드 내 모든 `transformers` 참조를 `transformer`로 업데이트

### Phase 2: 문서 및 구성 업데이트
- [ ] TASK 2-1: `specs/SPEC.md` 내의 모듈 경로 및 설명 업데이트
- [ ] TASK 2-2: `src/transformer/README.md` 및 프로젝트 루트 `README.md` 업데이트

### Phase 3: 빌드 및 테스트 검증
- [ ] TASK 3-1: 프로젝트 빌드 및 유닛 테스트 실행
- [ ] TASK 3-2: E2E 테스트 실행을 통한 최종 기능 검증
