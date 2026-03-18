# 계획: YAML 기반 Description 처리 및 멀티라인 지원

## 목적
현재 정규표현식 및 문자열 매칭 기반으로 구현된 `description` 필드 업데이트 로직을 `serde_yaml` 기반으로 교체합니다. 이를 통해 빌드 및 동기화(Sync) 과정에서 `|` 마커를 사용한 멀티라인 설명을 완벽하게 지원하도록 개선합니다.

## 배경
현재 `MdPatcher::update_description`은 원본 파일의 포맷(주석, 필드 순서 등) 보존을 위해 라인 단위 처리를 수행하며, 멀티라인 구조 감지 시 에러를 발생시킵니다. 하지만 100% 포맷 보존 요구사항이 완화됨에 따라, 보다 견고한 YAML 파서를 사용하여 멀티라인 데이터를 안전하게 처리할 수 있게 되었습니다.

## 주요 변경 사항

### 1. `src/syncer/patcher.rs`
- `MdPatcher::update_description` 로직 리팩토링:
    - `extract_frontmatter`를 사용하여 프론트매터 분리.
    - `serde_yaml`을 통해 YAML을 객체로 파싱 후 `description` 필드 업데이트.
    - 업데이트된 객체를 다시 YAML로 직렬화하여 마크다운 재구성.
- 기존의 멀티라인 감지 및 에러 반환 로직 제거.
- 멀티라인 입력에 대한 단위 테스트 추가.

### 2. 관련 문서 업데이트
- 멀티라인 제한 사항이 명시된 시스템 설계 및 명세 문서들을 최신화합니다.
    - `specs/design.md`
    - `specs/spec.md`
    - `src/syncer/README.md`

### 3. 검증 및 테스트
- `agb build`: 소스의 멀티라인 설명이 타겟 포맷에 맞게 잘 변환되는지 확인.
- `agb sync`: 타겟에서 수정된 멀티라인 설명이 소스로 정확히 역전파되는지 확인.
- 기존 멀티라인 실패 테스트(`tests/e2e_sync_multiline_test.rs`)를 성공 테스트로 전환.

## 단계별 실행 계획

### 1단계: 핵심 구현 및 단위 테스트
- **TASK 1**: `MdPatcher::update_description` 구현 수정 (YAML 기반)
- **TASK 2**: `MdPatcher` 단위 테스트 추가 (멀티라인 지원 확인)
- **TASK 3**: `extract_frontmatter` 단위 테스트 추가 (멀티라인 파싱 확인)

### 2단계: 통합 테스트 및 문서화
- **TASK 4**: 기존 멀티라인 실패 테스트(`tests/e2e_sync_multiline_test.rs`)를 성공 케이스로 전환
- **TASK 5**: 시스템 문서 업데이트 (디자인, 명세, 모듈 README)
