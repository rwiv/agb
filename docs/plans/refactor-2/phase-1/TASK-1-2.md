# Task 1.2: 프로젝트 전반의 임포트 경로 업데이트 및 검증

## 1. Objective (목표)

- `TransformedFile`의 위치 변경에 따라 프로젝트 내의 모든 임포트(Import) 구문을 업데이트합니다.
- 특히 `builder`와 `emitter` 모듈에서 발생하는 컴파일 에러를 해결합니다.

## 2. Context & Files (작업 범위)

- **수정할 파일:**
  - `src/builder/core.rs`
  - `src/emitter/core.rs`
  - `src/transformers/*.rs` (구현체들: gemini.rs, claude.rs 등)
  - `tests/*.rs` (테스트 코드)

## 3. Instructions (세부 지침)

### Step 1: 전역 검색 및 치환

`use crate::transformers::TransformedFile;` 또는 이와 유사한 구문을 검색하여 `use crate::resource::TransformedFile;`로 변경합니다.

### Step 2: 각 모듈 수정

- **Transformers 구현체**: `gemini.rs`, `claude.rs`, `opencode.rs` 등에서 반환 타입인 `TransformedFile`의 경로를 수정합니다.
- **Emitter**: `src/emitter/core.rs`에서 `transformers` 의존성을 제거하고 `resource`에서 모델을 가져오도록 수정합니다.
- **Builder**: 파이프라인 흐름상 변환 결과를 받아 emitter에 전달하는 부분의 타입을 맞춥니다.

### Step 3: 컴파일 검증

`cargo check`를 실행하여 모든 모듈이 정상적으로 연결되었는지 확인합니다.

## 4. Acceptance Criteria (검증 체크리스트)

1. 모든 소스 파일에서 `TransformedFile`을 `resource` 모듈로부터 정상적으로 가져오는가?
2. `emitter` 모듈의 `use` 구문에서 `transformers`에 대한 의존성이 사라졌는가?
3. 전체 프로젝트가 에러 없이 컴파일되는가?
