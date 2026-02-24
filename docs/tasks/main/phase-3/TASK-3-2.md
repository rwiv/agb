# Task 3.2: Gemini-cli용 TOML 변환 로직 구현

## 1. Objective (목표)

- `Transformer` 트레이트를 구현하여, `agb`의 리소스를 Gemini-cli 규격인 TOML 포맷으로 변환하는 `GeminiTransformer`를 개발합니다.
- 마크다운 파일의 내용은 `prompt` 필드로, JSON 메타데이터의 필드들은 TOML의 최상위 속성으로 정확히 매핑합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `docs/specs/SPEC.md` (Gemini-cli 변환 규칙 확인)
  - `docs/specs/TECH_SPEC.md` (사용 라이브러리 `toml` 확인)
- **생성 및 수정할 파일:**
  - `src/transformers/gemini.rs` (신규 생성: Gemini 전용 변환기 구현)
  - `src/transformers/mod.rs` (수정: `GeminiTransformer` 노출 및 팩토리 메서드 추가 가능성)

## 3. Instructions (세부 지침)

### Step 1: `GeminiTransformer` 구조체 및 트레이트 구현

`src/transformers/gemini.rs` 파일을 생성하고 `Transformer` 트레이트를 구현하세요.

- `transform` 메서드 구현:
    - `Resource`의 `metadata` (JSON)를 TOML의 기본 구조로 사용합니다.
    - `Resource`의 `content` (Markdown)를 TOML의 `prompt` 필드 값으로 할당합니다.
    - 결과물 경로 설정: 리소스 타입에 따라 `commands/[name].toml`, `agents/[name].toml` 등으로 설정합니다.

### Step 2: TOML 변환 로직 (Field Mapping)

- `toml` 크레이트를 사용하여 최종 문자열을 생성합니다.
- 예시 변환 구조:
    ```toml
    # 결과물 예시 (commands/foo.toml)
    model = "gemini-1.5-pro"
    description = "설명 내용"
    prompt = """
    여기에 마크다운 내용이 들어감
    """
    ```

### Step 3: 리소스 타입별 분기 처리

- `Command`, `Agent`, `Skill` 각각의 특성에 맞춰 파일 확장자 및 필드 구성을 조정합니다. (현재 Gemini-cli는 주로 `commands/`와 `agents/`를 지원함)

## 4. Constraints (제약 사항 및 금지 행동)

- 반드시 `toml` 라이브러리를 사용하여 직렬화를 수행하세요. 수동 문자열 조작은 지양합니다.
- 마크다운 내용 내의 따옴표나 특수 문자가 TOML의 멀티라인 문자열(`"""`) 내에서 올바르게 이스케이프되도록 처리하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `GeminiTransformer`가 `Transformer` 트레이트를 성공적으로 구현하였는가?
2. 마크다운 전문이 TOML의 `prompt` 필드에 누락 없이 삽입되는가?
3. JSON 메타데이터의 필드들이 TOML의 최상위 키로 올바르게 변환되는가?
4. 생성된 TOML 문자열이 문법적으로 유효하며 Gemini-cli에서 로드 가능한 구조인가?
5. `cargo test`를 통해 샘플 리소스의 변환 결과가 기대치와 일치하는지 확인하였는가?
