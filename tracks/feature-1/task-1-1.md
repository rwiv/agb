# Task 1.1: Command·Skill OpenAI 정책 파일 생성

## Overview

Codex 타겟의 Command와 Skill이 `disable-model-invocation: true` 최종 메타데이터를 가질 때 `agents/openai.yaml` 정책 파일을 생성합니다. 기존 `CodexTransformer::post_transform()`은 agent registry인 `config.toml`만 생성하므로, 이 단계에서 post-transform 산출물 목록에 정책 파일도 함께 추가합니다.

Skill은 source에 `agents/openai.yaml` extra가 있으면 생성 파일을 만들지 않습니다. 이 경우 기존 extras 복사 로직이 source 파일을 target으로 복사해야 합니다.

## File Scope

### Target Files

| 파일 | 설명 |
| --- | --- |
| `src/core/constants.rs` | 수정 — `agents/openai.yaml` 상대 경로 상수 추가 |
| `src/transformer/codex.rs` | 수정 — 정책 파일 생성 helper, `post_transform()` 확장, 단위 테스트 추가 |
| `specs/spec.md` | 수정 — Codex 변환 사양에 OpenAI 정책 파일 생성 규칙 추가 |
| `src/transformer/README.md` | 수정 — Codex Command·Skill의 추가 정책 파일 생성 설명 추가 |

### Reference Files

| 파일 | 설명 |
| --- | --- |
| `src/transformer/codex.rs` | 현재 Codex Command/Skill 경로 변환과 `post_transform()` 구조 확인 |
| `src/transformer/default.rs` | Command/Skill의 기본 SKILL.md frontmatter 생성 방식 확인 |
| `src/core/resource.rs` | `Resource`, `SkillData`, `ExtraFile`, `metadata()` 구조 확인 |
| `src/core/constants.rs` | Codex 출력 경로 상수(`DIR_AGENTS_SKILLS`, `SKILL_MD`) 확인 |
| `src/loader/merger.rs` | 최종 메타데이터가 Frontmatter, metadata map, 외부 `codex:` 섹션 병합 결과로 만들어지는 흐름 확인 |
| `src/loader/parser.rs` | Codex Skill extras target 경로가 `DIR_AGENTS_SKILLS` 기준으로 계산되는지 확인 |
| `specs/spec.md` | 4절 Codex 변환 사양 갱신 위치 확인 |
| `src/transformer/README.md` | 타겟별 특이사항 갱신 위치 확인 |

### External References

| 자료 | 설명 |
| --- | --- |
| OpenAI Codex Skills 문서 | `agents/openai.yaml`과 `policy.allow_implicit_invocation` 공식 동작 확인: https://developers.openai.com/codex/skills |

## Workflow

### Step 1: 공용 상대 경로 상수 추가 (`src/core/constants.rs`)

`agents/openai.yaml`은 build와 sync 양쪽에서 같은 의미로 사용되므로 공용 상수로 정의합니다.

```rust
// src/core/constants.rs
pub const CODEX_OPENAI_POLICY_RELATIVE_PATH: &str = "agents/openai.yaml";
```

정책 파일의 전체 target 경로는 `DIR_AGENTS_SKILLS`, 리소스 이름, 위 상대 경로를 조합해 만듭니다.

```rust
PathBuf::from(DIR_AGENTS_SKILLS)
    .join(resource_name)
    .join(CODEX_OPENAI_POLICY_RELATIVE_PATH)
```

### Step 2: Codex 정책 helper 추가 (`src/transformer/codex.rs`)

정책 파일 내용과 메타데이터 key는 Codex transformer 내부 구현 세부사항이므로 private constant로 둡니다.

```rust
const DISABLE_MODEL_INVOCATION: &str = "disable-model-invocation";
const OPENAI_POLICY_CONTENT: &str = "policy:\n  allow_implicit_invocation: false\n";

/// 최종 메타데이터가 OpenAI 정책 파일 생성을 요구하는지 확인한다.
fn requires_openai_policy(metadata: &serde_json::Value) -> bool {
    metadata
        .get(DISABLE_MODEL_INVOCATION)
        .and_then(|value| value.as_bool())
        == Some(true)
}
```

함수명은 실제 의미에 맞춰 조정할 수 있습니다. 중요한 제약은 boolean `true`만 허용하고 문자열 `"true"`나 `"false"`는 허용하지 않는 것입니다.

### Step 3: Skill source extra 존재 여부 확인 helper 추가 (`src/transformer/codex.rs`)

Skill은 source에 `agents/openai.yaml`이 있으면 생성 파일을 만들지 않아야 합니다. `SkillData.extras`에는 source extra와 target 경로가 들어 있으므로 이를 기준으로 판정합니다.

```rust
/// Skill source extras에 수동 OpenAI 정책 파일이 포함되어 있는지 확인한다.
fn has_source_openai_policy(skill: &SkillData) -> bool {
    skill.extras.iter().any(|extra| {
        extra
            .source
            .strip_prefix(&skill.base.source_path)
            .map(|relative| relative == Path::new(CODEX_OPENAI_POLICY_RELATIVE_PATH))
            .unwrap_or(false)
    })
}
```

source relative path 비교만 사용합니다. `extra.target.ends_with("agents/openai.yaml")` 같은 fallback은 `nested/agents/openai.yaml` 같은 파일을 source 정책 파일로 오인할 수 있으므로 사용하지 않습니다. 테스트 fixture도 실제 parser 산출물처럼 `skill.base.source_path`가 Skill 디렉터리를 가리키고, `extra.source`가 그 하위의 `agents/openai.yaml`을 가리키도록 구성합니다.

### Step 4: `post_transform()` 확장 (`src/transformer/codex.rs`)

기존 agent registry 생성 로직을 유지하면서 정책 파일 생성을 추가합니다. `post_transform()`은 agent가 없더라도 정책 파일만 반환할 수 있어야 합니다.

```rust
fn post_transform(&self, resources: &[&Resource]) -> Result<Vec<TransformedFile>> {
    let mut files = Vec::new();

    if let Some(config_file) = self.transform_agent_registry(resources)? {
        files.push(config_file);
    }

    files.extend(self.transform_openai_policy_files(resources));

    Ok(files)
}
```

`transform_agent_registry()`는 현재 `post_transform()` 내부의 `config.toml` 생성 로직을 옮긴 private helper입니다. 함수 분리는 필수는 아니지만, agent registry 생성과 정책 파일 생성을 분리하면 테스트와 유지보수가 쉬워집니다.

```rust
/// Codex agent registry인 config.toml 산출물을 생성한다.
fn transform_agent_registry(&self, resources: &[&Resource]) -> Result<Option<TransformedFile>> {
    // ...기존 post_transform()의 agents table 구성 로직
}
```

정책 파일 생성 로직은 다음 조건을 따라야 합니다.

```rust
/// Codex post-transform 단계에서 OpenAI 정책 파일 산출물을 생성한다.
fn transform_openai_policy_files(&self, resources: &[&Resource]) -> Vec<TransformedFile> {
    resources
        .iter()
        .filter_map(|resource| match resource {
            Resource::Command(data) if requires_openai_policy(&data.metadata) => {
                Some(openai_policy_file(&data.name))
            }
            Resource::Skill(skill)
                if requires_openai_policy(&skill.base.metadata)
                    && !has_source_openai_policy(skill) =>
            {
                Some(openai_policy_file(&skill.base.name))
            }
            _ => None,
        })
        .collect()
}
```

Rust formatter가 긴 guard를 정리하므로 실제 코드에서는 가독성을 우선해 helper를 추가해도 됩니다.

### Step 5: 정책 파일 생성 helper 추가 (`src/transformer/codex.rs`)

정책 파일은 정확히 아래 내용을 가져야 하며 마지막 개행을 포함합니다.

```rust
/// Codex OpenAI 정책 파일의 target 경로와 내용을 구성한다.
fn openai_policy_file(name: &str) -> TransformedFile {
    TransformedFile {
        path: PathBuf::from(DIR_AGENTS_SKILLS)
            .join(name)
            .join(CODEX_OPENAI_POLICY_RELATIVE_PATH),
        content: OPENAI_POLICY_CONTENT.to_string(),
    }
}
```

### Step 6: 단위 테스트 추가 (`src/transformer/codex.rs`)

기존 `test_codex_post_transform`은 agent registry를 검증하므로 유지합니다. 여기에 정책 파일 케이스를 추가합니다.

```rust
/// Command metadata가 boolean true이면 OpenAI 정책 파일을 생성한다.
#[test]
fn test_codex_post_transform_generates_openai_policy_for_command() {
    // ...
}

/// Skill metadata가 boolean true이고 source extra가 없으면 OpenAI 정책 파일을 생성한다.
#[test]
fn test_codex_post_transform_generates_openai_policy_for_skill_without_source_extra() {
    // ...
}

/// Skill source extra가 있으면 생성 정책 파일을 만들지 않는다.
#[test]
fn test_codex_post_transform_skips_generated_policy_for_skill_with_source_extra() {
    // ...
}

/// boolean true가 아닌 값은 정책 파일 생성 대상이 아니다.
#[test]
fn test_codex_post_transform_does_not_generate_policy_for_non_true_metadata() {
    // ...
}
```

테스트는 `TransformedFile.path`가 `../.agents/skills/[name]/agents/openai.yaml`인지 확인하고, `content`가 `OPENAI_POLICY_CONTENT`와 같은지 검증합니다. non-true 케이스에는 값 누락, boolean `false`, 문자열 `"true"`, 문자열 `"false"`를 포함합니다.

이 단계의 단위 테스트는 `CodexTransformer`에 이미 병합된 최종 metadata가 전달된다는 전제에서 정책 파일 생성 조건만 검증합니다. Frontmatter, metadata map, 외부 `codex:` 섹션의 우선순위가 실제 loader/merger 경로에서 올바르게 반영되는지는 Task 1.2의 Codex E2E 테스트에서 별도로 검증해야 합니다.

### Step 7: 명세와 모듈 README 갱신

`specs/spec.md`의 Codex 변환 사양에 다음 내용을 추가합니다.

- Command/Skill의 최종 metadata에 `disable-model-invocation: true`가 있으면 `agents/openai.yaml`을 생성한다.
- Skill source에 `agents/openai.yaml` extra가 있으면 생성하지 않고 extra 복사본을 사용한다.
- boolean `true`만 생성 조건이다.

`src/transformer/README.md`의 Codex 항목에도 동일한 target 경로와 생성 조건을 요약합니다.

## Success Criteria

- [x] `src/core/constants.rs`에 `CODEX_OPENAI_POLICY_RELATIVE_PATH` 상수가 추가되었다.
- [x] `CodexTransformer::post_transform()`이 agent registry와 정책 파일을 모두 반환할 수 있다.
- [x] Command metadata가 boolean `true`이면 `../.agents/skills/[name]/agents/openai.yaml` 파일이 반환된다.
- [x] Skill metadata가 boolean `true`이고 source extra가 없으면 `../.agents/skills/[name]/agents/openai.yaml` 파일이 반환된다.
- [x] Skill source extra `agents/openai.yaml`이 있으면 생성 정책 파일이 반환되지 않는다.
- [x] boolean `false`, 문자열 `"true"`, 문자열 `"false"` 값으로는 정책 파일이 생성되지 않는다.
- [x] `cargo test --lib transformer::codex`가 통과한다.
- [x] `specs/spec.md`와 `src/transformer/README.md`가 새 Codex 정책 파일 규칙을 설명한다.
