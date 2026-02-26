# Plan: 모듈 의존성 단순화 및 resource 통합 리팩토링

이 문서는 `emitter` 모듈을 `resource` 모듈로 통합하고, `TransformedFile` 모델의 위치를 조정하여 프로젝트의 의존성 구조를 최적화하는 리팩토링 여정을 관리합니다.

## Phase 1: 모델 이동 및 의존성 역전 (Model Migration) [DONE]
*   **Task 1.1: `TransformedFile` 이동 및 `transformers` 의존성 제거** [DONE]
*   **Task 1.2: 프로젝트 전반의 임포트 경로 업데이트 및 검증** [DONE]

## Phase 2: Emitter 모듈 통합 (Module Integration) [DONE]
*   **Task 2.1: `emitter` 로직을 `resource` 모듈로 이동 및 통합** [DONE]
    *   *성공 기준:* `src/emitter/core.rs`의 로직이 `src/resource/emitter.rs`로 이동하고 `resource` 모듈의 일부로 작동한다.
*   **Task 2.2: 기존 `emitter` 모듈 제거 및 `builder` 오케스트레이션 수정** [DONE]
    *   *성공 기준:* 프로젝트 루트의 `src/emitter` 디렉토리가 삭제되고, `builder`가 새 위치의 `Emitter`를 사용하여 빌드 파이프라인을 완성한다.

## Phase 3: 최종 검증 (Verification) [DONE]
*   **Task 3.1: 리팩토링 후 전체 테스트 및 빌드 확인** [DONE]
    *   *성공 기준:* 기존의 E2E 테스트 및 유닛 테스트가 모두 통과하며, 의존성 그래프가 의도한 대로 단순화된다.
