# Task 3.1: Transformer 트레이트 정의

## 1. Objective (목표)

- 에이전트별(Gemini, Claude 등)로 서로 다른 변환 로직을 일관된 인터페이스로 다루기 위한 `Transformer` 트레이트를 정의합니다.
- 변환 결과를 담는 공통 구조체(`TransformedFile`)를 설계하여 이후 단계의 `Emitter`가 타겟 에이전트에 구애받지 않고 파일을 쓸 수 있도록 합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `docs/specs/TECH_SPEC.md` (Transformer 트레이트 설계 확인)
  - `src/core/resource.rs` (Resource 모델 구조 확인)
- **생성 및 수정할 파일:**
  - `src/transformers/base.rs` (신규 생성: 트레이트 및 공통 모델 정의)
  - `src/transformers/mod.rs` (수정: 모듈 구조 재편 및 base 노출)

## 3. Instructions (세부 지침)

### Step 1: `base.rs`에 핵심 인터페이스 정의

`src/transformers/base.rs` 파일을 생성하고, 에이전트별 변환 로직을 추상화하는 공통 인터페이스를 설계하세요.

- **`TransformedFile` 구조체**: 빌드 결과물의 경로(`path`)와 내용(`content`)을 담습니다.
- **`Transformer` 트레이트**: `transform`과 `transform_root_prompt` 메서드를 정의합니다.
- **`TransformerError` 열거형**: `thiserror`를 사용하여 변환 중 발생할 수 있는 에러(메타데이터 누락 등)를 정의합니다.

### Step 2: `mod.rs` 리팩토링 및 팩토리 함수 작성

- `src/transformers/mod.rs`에서 `base` 모듈을 선언하고 필요한 타입을 `pub use`로 재노출합니다.
- `get_transformer` 함수를 작성하여 `BuildTarget`에 따라 적절한 변환기 인스턴스(`Box<dyn Transformer>`)를 반환하도록 구현합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 특정 에이전트(Gemini 등)에 종속된 로직을 이 트레이트에 직접 작성하지 마세요. 오직 추상화된 인터페이스만 정의합니다.
- `src/transformers/mod.rs`에서 `Transformer` 트레이트를 `pub`으로 노출하여 외부 모듈에서 접근 가능하게 하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `Transformer` 트레이트가 `TECH_SPEC.md`에 명시된 메서드 시그니처를 정확히 포함하고 있는가?
2. `TransformedFile` 구조체가 파일 경로와 내용을 모두 포함하고 있는가?
3. `src/transformers/mod.rs` 파일이 에러 없이 컴파일되는가?
4. `Resource` 객체를 인자로 받아 `Result<TransformedFile>`을 반환하는 구조가 확립되었는가?
