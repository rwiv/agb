# Plan: Project Restructuring (Resource to Core, Loader, Builder)

이 문서는 `resource` 모듈을 해체하고 `core`, `loader`, `builder`로 리팩토링하는 전체 과정을 관리합니다. 각 테스크는 `plans/refactor-6/phase-X/TASK-X-Y.md` 문서에 상세 가이드를 가집니다.

## Phase 1: `core` 모듈 구축 및 공용 모델 이주 (Core Module & Models)
*   **Task 1.1: `src/core` 모듈 생성 및 모델 이동**
    *   *성공 기준:* `src/resource/model.rs`의 내용을 `src/core/model.rs`로 이동하고, `src/core/mod.rs`에서 적절히 export한다.
*   **Task 1.2: `transformer` 모듈의 의존성 업데이트**
    *   *성공 기준:* `src/transformer/` 내의 모든 파일에서 `crate::resource` 참조를 `crate::core`로 변경하고 빌드에 성공한다.

## Phase 2: `loader` 모듈 최상위 승격 및 독립 (Loader Promotion)
*   **Task 2.1: `src/resource/loader`를 `src/loader`로 이동**
    *   *성공 기준:* `src/resource/loader/` 디렉토리를 `src/loader/`로 이동하고, 내부 파일들(`filter.rs`, `parser.rs`, `resolver.rs`)의 의존성 및 모듈 선언을 업데이트한다.
*   **Task 2.2: `loader` 모듈의 `core` 참조 및 테스트 수정**
    *   *성공 기준:* `src/loader` 내부에서 `crate::resource` 참조를 제거하고, `src/loader` 내의 유닛 테스트가 모두 통과한다.

## Phase 3: `builder` 모듈 재구조화 및 최종 정리 (Builder Restructuring)
*   **Task 3.1: `registry.rs` 및 `emitter.rs`를 `builder` 서브 모듈로 이동**
    *   *성공 기준:* `src/resource/registry.rs`와 `src/resource/emitter.rs`를 `src/builder/` 하위로 이동하고, `builder`의 서브 모듈로 등록한다.
*   **Task 3.2: `builder` 모듈의 전체 의존성 및 워크플로우 업데이트**
    *   *성공 기준:* `src/builder/mod.rs`에서 `loader`, `registry`, `emitter`, `transformer`를 통합하여 빌드 프로세스를 재구성하고 모든 E2E 테스트를 통과한다.
*   **Task 3.3: `src/resource` 모듈 삭제 및 문서 업데이트**
    *   *성공 기준:* 더 이상 사용되지 않는 `src/resource/` 모듈을 삭제하고, `README.md` 및 `SPEC.md` 등의 문서를 새로운 구조에 맞게 업데이트한다.
