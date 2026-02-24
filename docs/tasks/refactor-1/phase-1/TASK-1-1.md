# Task 1.1: `core` 모듈을 `resource`로 리네이밍

## 1. Objective (목표)

- `core`라는 모호한 이름을 프로젝트의 도메인을 명확히 나타내는 `resource`로 변경합니다.
- 모듈 구조를 직관적으로 개선하여 코드 가독성과 유지보수성을 높입니다.

## 2. Context & Files (작업 범위)

- **이동 및 리네이밍:**
  - `src/core/` -> `src/resource/`
- **수정할 파일 (참조 업데이트):**
  - `src/main.rs` (모듈 선언 및 사용처)
  - `src/builder/core.rs` (리소스 스캔 및 등록 로직)
  - `src/transformers/base.rs` (리소스 모델 참조)
  - `src/transformers/gemini.rs`, `claude.rs`, `opencode.rs` (리소스 모델 참조)
  - `src/resource/mod.rs` (내부 모듈 구조 확인)

## 3. Instructions (세부 지침)

### Step 1: 디렉터리 및 파일 리네이밍
`src/core` 디렉터리 이름을 `src/resource`로 변경합니다.

### Step 2: 모듈 선언 업데이트
`src/main.rs` 또는 최상위 모듈 선언부에서 `pub mod core;`를 `pub mod resource;`로 수정합니다.

### Step 3: 프로젝트 전체의 참조 수정
코드 내의 모든 `crate::core` 참조를 `crate::resource`로 일괄 업데이트합니다.
- `use crate::core::resource::Resource;` -> `use crate::resource::resource::Resource;`
- (참고: `src/resource/resource.rs` 파일명도 구조에 따라 `src/resource/model.rs` 등으로 변경을 검토할 수 있으나, 본 태스크에서는 모듈명 변경에 집중합니다.)

### Step 4: 문서 업데이트
`src/resource/README.md` 및 파일 내의 주석에서 `core`로 언급된 부분을 `resource`로 수정합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 모듈 이름 외의 내부 로직(기능)은 변경하지 않습니다.
- 기존의 가시성(`pub`, `pub(crate)`) 설정을 그대로 유지하여 의존성 문제가 발생하지 않도록 합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `src/core` 디렉터리가 제거되고 `src/resource` 디렉터리가 생성되었는가?
2. `cargo build`가 에러 없이 성공하는가?
3. 기존의 모든 테스트(`cargo test`)가 통과하는가?
4. 프로젝트 내에 `crate::core`라는 문자열이 더 이상 존재하지 않는가? (테스트 fixture 제외)
