# Refactor 7: 상수화 및 구조적 개선

## 1. 개요
프로젝트 전반에 걸친 매직 넘버 상수화, 타입 임포트 최적화, 그리고 유틸리티 함수의 위치 재조정을 통한 코드 구조의 현대화 및 가독성 확보.

## 2. 주요 작업 내용
- `src/core/mod.rs` 또는 `src/core/constants.rs`에 파일/디렉터리 이름 상수 정의.
- `src/transformer/gemini.rs`의 `json_to_toml` 함수를 `src/utils/toml.rs`로 이동.
- 소스 코드 전반의 `crate::core::...` 전체 경로 참조를 `use` 문을 통한 타입명 참조로 수정.
- 빌드 결과물 경로 설정 시 상수를 사용하도록 변경.

## 3. 마일스톤 (Milestones)

### Phase 1: 기반 상수의 정의 및 타입 임포트 정리
- [ ] TASK 1-1: `src/core/mod.rs`에 상수 정의 및 `src/core` 내 타입 임포트 정리
- [ ] TASK 1-2: `src/builder` 및 `src/loader` 모듈 내 상수 적용 및 타입 임포트 정리
- [ ] TASK 1-3: `src/transformer` 모듈 내 상수 적용 및 타입 임포트 정리

### Phase 2: 유틸리티 함수 이동 및 리팩토링
- [ ] TASK 2-1: `src/utils/toml.rs` 생성 및 `json_to_toml` 함수 이동
- [ ] TASK 2-2: `src/transformer/gemini.rs`에서 `src/utils/toml::json_to_toml`을 사용하도록 수정 및 정리

### Phase 3: 테스트 및 최종 검증
- [ ] TASK 3-1: 기존 유닛 테스트 및 E2E 테스트 실행을 통한 정합성 검증
- [ ] TASK 3-2: 상수를 통한 디렉터리 이름 변경 테스트 (선택 사항, 로컬 검증)

## 4. 관련 문서
- [DESIGN.md](./DESIGN.md)
