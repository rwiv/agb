# Task 3.1: 리팩토링 후 전체 테스트 및 빌드 확인

## 1. Objective (목표)

- 리팩토링이 기존 기능에 영향을 주지 않았음을 보증합니다.
- 의존성 단순화가 의도대로 반영되었는지 최종 확인합니다.

## 2. Context & Files (작업 범위)

- **대상:** 프로젝트 전체 테스트 세트 (`tests/*.rs`)
- **도구:** `cargo test`, `cargo build`

## 3. Instructions (세부 지침)

### Step 1: 전체 테스트 실행

`cargo test`를 실행하여 모든 E2E 테스트와 유닛 테스트가 통과하는지 확인합니다.

### Step 2: 의존성 그래프 검토 (선택 사항)

`cargo modules graph` (설치되어 있다면) 등을 통해 의존성 구조가 아래와 같이 변경되었는지 확인합니다.
- `builder` -> `resource`, `transformers`
- `transformers` -> `resource`
- `resource` 내부에 `emitter` 포함

### Step 3: 최종 빌드 확인

`cargo build --release`를 통해 실제 바이너리가 정상적으로 생성되는지 확인합니다.

## 4. Acceptance Criteria (검증 체크리스트)

1. 모든 테스트가 Fail 없이 통과하는가?
2. 빌드 결과물에 이상이 없는가?
3. 코드 내에 사용되지 않는 구문(Unused imports)이나 경고가 없는가?
