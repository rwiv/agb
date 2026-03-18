# Plan: Preserve Frontmatter Order in Build and Sync (Feature 16)

이 문서는 `agb` 시스템에서 `build` 및 `sync` 수행 시 마크다운 파일의 프론트매터(Frontmatter) 키 순서가 원본과 동일하게 유지되지 않고 변형되는 문제를 해결하기 위한 개발 여정을 관리합니다. 
해결 방안으로는 복잡한 정규표현식 부분 패치 대신, `serde_json` 라이브러리의 `preserve_order` 피처(Feature)를 활성화하여 안전하고 깔끔하게 키 순서를 보존하는 방식을 채택합니다.
이 계획에는 의존성 설정 변경 및 관련 기술 문서(specs) 갱신 작업이 포함되어 있습니다.

## Phase 1: `serde_json` 순서 보존 피처 적용 (Apply `preserve_order`)
*   **Task 1.1: `Cargo.toml` 의존성 업데이트**
    *   *성공 기준:* `Cargo.toml` 파일의 `serde_json` 의존성에 `features = ["preserve_order"]`가 추가되고 빌드가 성공한다.
*   **Task 1.2: 프론트매터 순서 보존 검증 테스트**
    *   *성공 기준:* 빌드 및 동기화 동작 후 생성된 마크다운 파일의 프론트매터 순서가 원본 파일과 동일하게 유지되는 것을 증명하는 유닛 또는 통합 테스트 케이스가 작성되어 통과한다.

## Phase 2: 기술 명세 문서 갱신 (Documentation Updates)
*   **Task 2.1: `spec.md` 및 `design.md` 갱신**
    *   *성공 기준:* 문서 상의 동기화 패치 알고리즘과 메타데이터 처리 규격에, 프론트매터 파싱 및 직렬화 시 순서가 보존된다는 내용과 `preserve_order` 피처 사용에 대한 기술적 설명이 반영된다.
