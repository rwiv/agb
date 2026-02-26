# Plan: agb Refactoring (Circular Dependency & Loader Responsibilities)

이 마스터 리스트는 `builder`와 `transformer` 모듈 간의 순환 의존성 해결 및 `resource::loader` 모듈의 책임 분산을 위한 태스크를 관리합니다.

## Phase 1: BuildTarget 이동 및 순환 의존성 제거
*   **Task 1.1: BuildTarget 이동 및 임포트 수정**
    *   *성공 기준:* `BuildTarget`이 `resource` 모듈로 이동하고, `transformer`와 `builder` 모듈의 임포트가 수정되어 순환 의존성이 해결된다.
*   **Task 1.2: 검증 및 정리**
    *   *성공 기준:* `cargo test`를 포함한 모든 정적 분석 및 테스트가 통과하며, 순환 의존성이 존재하지 않음을 확인한다.

## Phase 2: ResourceLoader 책임 분산 및 ResourceParser 도입
*   **Task 2.1: MetadataParser를 ResourceParser로 리네이밍 및 기능 확장**
    *   *성공 기준:* `MetadataParser`가 `ResourceParser`로 이름이 바뀌고, `ResourceLoader`의 `parse_resource` 로직을 흡수하여 독립적으로 리소스 객체를 생성할 수 있게 된다.
*   **Task 2.2: ResourceLoader 오케스트레이션 최적화 및 최종 검증**
    *   *성공 기준:* `ResourceLoader`가 `scan`, `resolve`, `parse` 단계를 순차적으로 호출하는 단순한 구조로 변경되고, 모든 테스트가 통과한다.
