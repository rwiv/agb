# Task 3.3: 전역 시스템 프롬프트(AGENTS.md) 변환

## 1. Objective (목표)

- 프로젝트 루트에 위치한 전역 지침 파일(`AGENTS.md`)을 타겟 에이전트가 인식하는 메인 메모리 파일로 변환합니다.
- Gemini-cli의 경우 `GEMINI.md`로, 향후 지원할 Claude-code의 경우 `CLAUDE.md` 등으로 변환되는 로직을 구현합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/PRD.md` (전역 지침 관리 정책 확인)
  - `specs/SPEC.md` (빌드 단계에서의 Clean & Merge 단계 확인)
- **생성 및 수정할 파일:**
  - `src/transformers/gemini.rs` (수정: `transform_root_prompt` 구현)
  - `src/transformers/mod.rs` (수정: 인터페이스 확인)

## 3. Instructions (세부 지침)

### Step 1: `GeminiTransformer::transform_root_prompt` 구현

- 입력받은 `AGENTS.md`의 내용을 그대로 사용하거나, 필요시 Gemini-cli에 최적화된 서술 방식으로 가공합니다.
- 결과 파일의 이름을 `GEMINI.md`로 지정한 `TransformedFile` 객체를 반환합니다.

### Step 2: 메타데이터 및 헤더 처리

- 만약 `AGENTS.md`에 특정 타겟에만 적용되어야 하는 섹션이 있다면 이를 필터링하는 로직을 고려합니다. (현재는 단순 복사/이름 변경을 기본으로 함)

### Step 3: 통합 테스트 케이스 추가

- 루트에 `AGENTS.md`가 있을 때 `GeminiTransformer`를 거치면 `GEMINI.md`라는 이름과 원본 내용을 가진 결과물이 생성되는지 검증하는 테스트를 작성하세요.

## 4. Constraints (제약 사항 및 금지 행동)

- 플러그인 내부에 `GEMINI.md`가 존재하는지 검사하고, 존재한다면 PRD 제약 사항에 따라 에러를 발생시켜야 합니다. (이 로직은 Loader 또는 Registry 단계에서 수행될 수도 있으나, Transformer 단계에서도 확인이 필요할 수 있음)
- 파일의 내용뿐만 아니라 **파일 이름(Target Filename)**이 각 에이전트 규격에 정확히 맞는지 확인하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `AGENTS.md` 내용이 `GEMINI.md` 파일로 변환되도록 경로가 올바르게 설정되었는가?
2. 변환된 파일의 내용이 원본의 의미를 훼손하지 않고 유지되는가?
3. 서로 다른 타겟(Gemini vs Claude)에 대해 서로 다른 파일 이름이 생성되는지 확인하였는가?
4. `cargo check` 시 타입 에러나 미구현 메서드 경고가 없는가?
