# Task 5.1: Claude-code용 Markdown 변환기 구현

## 1. Objective (목표)

- `Transformer` 트레이트를 구현하여 Claude-code 에이전트 규격에 맞는 리소스 변환기를 개발합니다.
- Markdown 내용과 JSON 메타데이터를 결합하여 Claude가 인식하기 최적화된 단일 마크다운 파일 구조를 생성합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (Claude-code 변환 규칙 확인)
  - `src/transformers/mod.rs` (Transformer 트레이트 정의 확인)
- **생성 및 수정할 파일:**
  - `src/transformers/claude.rs` (신규 생성: Claude용 변환 로직 구현)
  - `src/transformers/mod.rs` (수정: Claude 변환기를 Factory에 등록)

## 3. Instructions (세부 지침)

### Step 1: `ClaudeTransformer` 구조체 및 트레이트 구현

`src/transformers/claude.rs`에서 `Transformer` 트레이트를 구현하세요.

- **`transform()` 구현:** 
  - `Resource` 객체의 JSON 메타데이터를 마크다운의 **Frontmatter (YAML)** 형식으로 변환합니다.
  - Frontmatter 아래에 리소스의 마크다운 본문을 결합합니다.
  - 결과물을 `TransformedFile` 객체로 반환합니다.
- **`transform_root_prompt()` 구현:**
  - `AGENTS.md`의 내용을 `CLAUDE.md`라는 이름의 파일로 변환합니다.

### Step 2: Frontmatter 구성 규칙

Claude-code용 결과물 상단에 아래와 같은 형식을 포함해야 합니다.

```markdown
---
description: "메타데이터의 description 값"
parameters: { ...메타데이터의 나머지 필드... }
---

# 본문 내용...
```

### Step 3: Transformer Factory 연동

- `src/transformers/mod.rs`에서 `BuildTarget::ClaudeCode`가 선택되었을 때 `ClaudeTransformer` 인스턴스를 반환하도록 수정합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- Claude-code 규격은 파일 확장이 `.md`여야 합니다.
- 메타데이터 변환 시 `serde_yaml`을 사용하여 표준적인 Frontmatter 형식을 유지하세요.
- Gemini용 TOML 변환 로직과 혼용되지 않도록 모듈을 엄격히 분리하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `target: claude-code` 설정 시 결과물이 `.md` 확장자로 생성되는가?
2. 생성된 마크다운 파일 상단에 YAML Frontmatter가 올바른 문법으로 삽입되어 있는가?
3. 메타데이터의 내용과 본문 마크다운 내용이 누락 없이 병합되었는가?
4. `AGENTS.md`가 `CLAUDE.md`로 정확히 변환되어 출력되는가?
