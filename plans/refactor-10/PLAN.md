# Plan: App Module Refactoring

이 문서는 `src/main.rs` 로직을 `app` 모듈로 분리하여 실행 컨텍스트를 캡슐화하는 리팩토링 과정을 관리합니다. 각 테스크는 `plans/refactor-10/phase-X/TASK-X-Y.md` 문서에 상세 가이드를 가집니다.

## Phase 1: `lib.rs` 구축 및 `app` 모듈 기반 마련 (Setup Lib & App Module)
*   **Task 1.1: `src/lib.rs` 생성 및 `mod` 선언 이동**
    *   *성공 기준:* `src/main.rs`의 `mod` 선언들을 `src/lib.rs`로 이동하고, `main.rs`는 `lib.rs`를 참조하도록 수정한다.
*   **Task 1.2: `src/app` 모듈 생성 및 `AppContext` 정의**
    *   *성공 기준:* `src/app/mod.rs` 파일을 생성하고, `AppContext` 구조체와 초기화 로직(`init`)을 구현한다.

## Phase 2: 실행 로직 이동 및 `App` 구조체 구현 (Move Execution Logic)
*   **Task 2.1: `App` 구조체 구현 및 명령어 분기 로직 이동**
    *   *성공 기준:* `src/app/mod.rs`에 `App` 구조체를 정의하고, `main.rs`의 `Build`/`Sync` 명령어 처리 로직을 `App`의 메서드로 이동한다.
*   **Task 2.2: `main.rs` 축소 및 `App::run` 호출**
    *   *성공 기준:* `src/main.rs`에서 `clap` 파싱 결과만 `App::run`에 전달하고, 모든 비즈니스 로직이 제거된 상태에서 기존 기능이 동일하게 작동한다.

## Phase 3: 마무리 및 검증 (Cleanup & Validation)
*   **Task 3.1: 기존 테스트 및 의존성 업데이트**
    *   *성공 기준:* 모든 E2E 테스트(`tests/`)가 리팩토링된 구조에서도 정상적으로 통과한다.
*   **Task 3.2: 라이브러리 기반 통합 테스트 추가**
    *   *성공 기준:* `App` 구조체를 직접 사용하는 통합 테스트를 추가하여, 바이너리 실행 없이도 로직 검증이 가능함을 증명한다.
*   **Task 3.3: 문서 업데이트 및 최종 정리**
    *   *성공 기준:* `README.md` 및 모듈 문서에서 변경된 구조를 반영하고 리팩토링을 완료한다.
