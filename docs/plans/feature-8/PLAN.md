# Plan: JSON 메타데이터 지원 제거 (Feature-8)

JSON 메타데이터 지원을 완전히 제거하고 YAML(`.yaml`, `.yml`)로 단일화하는 작업을 수행합니다.

## Phase 1: 소스 코드 및 로직 수정
- [ ] TASK-1-1: `ResourcePathResolver`에서 JSON 확장자 제거 및 중복 검증 확인
- [ ] TASK-1-2: `ResourceParser`에서 JSON 파싱 로직 제거 및 정리

## Phase 2: 테스트 환경 및 픽스처 업데이트
- [ ] TASK-2-1: `tests/fixtures` 내의 모든 `.json` 파일을 `.yaml`로 변환
- [ ] TASK-2-2: 유닛 테스트 및 E2E 테스트 내의 JSON 관련 코드 수정

## Phase 3: 문서 최신화 및 최종 검증
- [ ] TASK-3-1: `SPEC.md` 및 `README.md` 등 문서에서 JSON 관련 내용 삭제
- [ ] TASK-3-2: 전체 빌드 및 테스트 수행을 통한 최종 검증
