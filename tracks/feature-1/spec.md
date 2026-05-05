# Specification: Codex OpenAI 정책 파일 생성

## Overview

Codex 타겟에서 Command 또는 Skill의 최종 메타데이터에 `disable-model-invocation: true`가 존재하면 OpenAI 정책 파일을 생성합니다. 생성 대상 파일은 각 Codex SKILL 출력 디렉터리 하위의 `agents/openai.yaml`입니다.

```yaml
policy:
  allow_implicit_invocation: false
```

이 기능은 Codex 타겟에만 적용됩니다. 다른 타겟(`gemini-cli`, `claude-code`, `opencode`)의 출력 구조와 sync 동작은 변경하지 않습니다.

OpenAI 공식 Codex Skills 문서는 `agents/openai.yaml`의 `policy.allow_implicit_invocation` 값으로 skill의 암묵 호출 여부를 제어한다고 설명합니다. `allow_implicit_invocation: false`는 사용자가 명시적으로 `$skill`을 호출한 경우에는 동작하지만, 설명 기반의 자동 선택은 막습니다. 참고: https://developers.openai.com/codex/skills

## Requirements

### 1. 메타데이터 판정 기준

정책 파일 생성 여부는 `loader::merger::MetadataMerger`가 반환한 최종 메타데이터를 기준으로 판단합니다.

- Frontmatter, metadata map, 외부 메타데이터(`codex:` 섹션) 병합 결과를 기준으로 한다.
- `disable-model-invocation` 값이 YAML boolean `true`인 경우에만 생성 대상이다.
- 문자열 `"true"`, 문자열 `"false"`, 값 누락, `false`, `null`은 생성 대상이 아니다.

### 2. Command 정책 파일 생성

Codex 타겟에서 Command는 `disable-model-invocation: true`이면 항상 정책 파일을 생성합니다.

- 대상 경로: output-dir(`.codex/`) 기준 `../.agents/skills/[command-name]/agents/openai.yaml`
- 실제 경로: 프로젝트 루트 기준 `.agents/skills/[command-name]/agents/openai.yaml`
- `build`와 `build --clean` 모두 동일한 결과를 생성해야 한다.
- 같은 이름의 Codex Command 출력 디렉터리는 기존 clean 로직으로 삭제되므로, 이전 빌드 산출물이나 수동 target 파일에 의존하지 않는다.

### 3. Skill 정책 파일 생성

Codex 타겟에서 Skill은 `disable-model-invocation: true`이고 source skill 디렉터리에 `agents/openai.yaml` extra가 없을 때만 정책 파일을 생성합니다.

- 대상 경로: output-dir(`.codex/`) 기준 `../.agents/skills/[skill-name]/agents/openai.yaml`
- source 기준 존재 확인 경로: `[source]/[plugin]/skills/[skill-name]/agents/openai.yaml`
- source에 `agents/openai.yaml`이 있으면 생성 파일로 덮지 않고 기존 extra 파일을 복사한다.
- source에 `agents/openai.yaml`이 있고 메타데이터가 `true`가 아니어도 기존 extra 복사 정책은 유지한다.

### 4. Sync 정책

생성 파일과 source extra 파일은 sync에서 다르게 취급합니다.

- Command의 생성 정책 파일은 Command sync 대상이 아니다. Command sync는 기존처럼 `SKILL.md` 본문과 description만 소스로 반영한다.
- Skill에서 source에 없어서 생성된 `agents/openai.yaml`은 `atb sync` 때 source로 추가하지 않는다.
- Skill에서 source에 원래 존재하던 `agents/openai.yaml`은 일반 extra 파일처럼 sync 대상이다.
  - target에서 내용이 바뀌면 source extra를 업데이트한다.
  - target에서 삭제되면 source extra를 삭제한다.
  - source에 없던 파일이 사용자가 target에 새로 만든 수동 extra라면 기존 ExtraSyncer 정책을 따른다. 단, 생성 파일로 판정되는 `agents/openai.yaml`은 제외한다.

### 5. Exclude 충돌 검증

Codex 타겟에서 빌드 대상 Skill의 source 디렉터리에 `agents/openai.yaml`이 실제로 존재하지만 `toolkit.yaml`의 `exclude` 패턴에 의해 제외된다면 즉시 에러를 반환합니다.

- 검증 대상은 `toolkit.yaml`의 `resources.skills`에 명시되어 registry에 등록된 Skill로 한정한다.
- 선택되지 않은 Skill의 `agents/openai.yaml`이 exclude에 걸려도 build/sync를 실패시키지 않는다.
- 검증 경로: `[source]/[plugin]/skills/[skill-name]/agents/openai.yaml`
- 패턴 매칭 기준: source root 기준 상대 경로가 `exclude` 패턴과 매칭되는지 확인한다.
- 에러 메시지에는 plugin/name, source root 기준 상대 경로, 매칭된 exclude 패턴을 포함한다.
- 이 검증은 `SkillData.extras`만으로 source 정책 파일 존재 여부를 판단하면 exclude 때문에 source 파일을 build 산출물로 오인할 수 있는 문제를 방지하기 위한 fail-fast 규칙이다.

## Directory Structure

### Command

```text
프로젝트 루트/
├── .codex/
│   └── toolkit.yaml
└── .agents/
    └── skills/
        └── my-command/
            ├── SKILL.md
            └── agents/
                └── openai.yaml
```

### Skill: source extra 없음

```text
source/
└── my_plugin/
    └── skills/
        └── my_skill/
            └── SKILL.md

프로젝트 루트/
└── .agents/
    └── skills/
        └── my_skill/
            ├── SKILL.md
            └── agents/
                └── openai.yaml   # build 산출물, sync 시 source로 역전파하지 않음
```

### Skill: source extra 있음

```text
source/
└── my_plugin/
    └── skills/
        └── my_skill/
            ├── SKILL.md
            └── agents/
                └── openai.yaml   # source extra, sync 대상

프로젝트 루트/
└── .agents/
    └── skills/
        └── my_skill/
            ├── SKILL.md
            └── agents/
                └── openai.yaml   # source extra 복사본
```

## Implementation Strategy

### 1. 공용 상수

`agents/openai.yaml` 경로는 transformer와 syncer가 함께 참조해야 하므로 공용 상수로 분리합니다.

```rust
// src/core/constants.rs
pub const CODEX_OPENAI_POLICY_RELATIVE_PATH: &str = "agents/openai.yaml";
```

정책 파일 내용은 Codex transformer 내부 구현 세부사항이므로 `src/transformer/codex.rs`의 private constant로 둘 수 있습니다.

```rust
const OPENAI_POLICY_CONTENT: &str = "policy:\n  allow_implicit_invocation: false\n";
const DISABLE_MODEL_INVOCATION: &str = "disable-model-invocation";
```

### 2. Build 생성 위치

`CodexTransformer::post_transform()`에서 기존 `config.toml` 생성과 함께 정책 파일을 추가 생성합니다.

- `post_transform()`은 전체 리소스를 볼 수 있으므로 Command와 Skill을 한 번에 판정할 수 있다.
- Command와 Skill의 기본 `SKILL.md` 변환 로직은 유지한다.
- 정책 파일은 `TransformedFile`로 반환하여 Emitter가 일반 변환 산출물처럼 기록하게 한다.
- Skill source extra가 존재하는 경우에는 `TransformedFile`을 만들지 않고 기존 `ExtraFile` 복사에 맡긴다.

### 3. Sync 제외 위치

생성된 Skill 정책 파일을 source extra로 역전파하지 않으려면 `ExtraSyncer`가 특정 relative path를 무시할 수 있어야 합니다.

- `Transformer` trait에 기본 구현을 가진 ignore hook을 추가한다.
- `CodexTransformer`는 생성 파일로 판정되는 Skill에 대해서만 `agents/openai.yaml`을 ignore path로 반환한다.
- `Syncer`는 Skill extra sync 호출 시 해당 ignore path를 `ExtraSyncer`에 전달한다.
- source extra가 이미 있는 Skill은 ignore path를 반환하지 않으므로 기존처럼 sync된다.

### 4. Exclude 충돌 검증 위치

source에 실제로 존재하는 `agents/openai.yaml`이 `exclude`에 걸리는 경우는 loader scan 단계에서 extras 목록에서 빠지므로, transformer 단계까지 내려가면 source extra와 생성 파일을 구분할 수 없습니다. 따라서 `AppContext::init()`에서 registry에 빌드 대상 리소스를 등록한 뒤 Codex 타겟에 한해 fail-fast 검증을 수행합니다.

- 검증은 `registry.all_resources()`의 `Resource::Skill`만 대상으로 한다.
- `skill.base.source_path.join(CODEX_OPENAI_POLICY_RELATIVE_PATH)`가 실제 파일이고, source root 기준 상대 경로가 `exclude_patterns` 중 하나와 매칭되면 에러를 반환한다.
- 선택되지 않은 Skill은 registry에 등록되지 않으므로 검증 대상에서 제외된다.

## Testing Strategy

- `src/transformer/codex.rs` 단위 테스트
  - Command metadata가 boolean `true`이면 `agents/openai.yaml` 생성
  - Command metadata가 누락, `false`, 문자열 `"true"`, 문자열 `"false"`이면 생성하지 않음
  - Skill metadata가 boolean `true`이고 source extra가 없으면 생성
  - Skill metadata가 boolean `true`여도 source extra가 있으면 생성하지 않음
  - 기존 agent registry `config.toml` 생성과 정책 파일 생성이 함께 동작
- `src/syncer/extra.rs` 단위 테스트
  - ignore path에 포함된 target 파일은 Add/Update 액션을 만들지 않음
  - ignore path에 포함된 source 파일은 Delete 액션을 만들지 않음
  - ignore path가 아닌 기존 source extra는 Update/Delete 액션을 유지
- `src/app/context.rs` 검증 테스트
  - Codex 빌드 대상 Skill의 source `agents/openai.yaml`이 exclude에 걸리면 `AppContext::init()` 실패
  - 선택되지 않은 Skill의 source `agents/openai.yaml`이 exclude에 걸려도 `AppContext::init()` 성공
- Codex E2E 테스트
  - `build`: Command와 Skill 정책 파일 생성 확인
  - `build`: 외부 메타데이터 파일의 `codex:` 섹션에서 boolean `true`가 최종 메타데이터에 반영되면 정책 파일 생성 확인
  - `build`: Frontmatter boolean `true`가 외부 `codex:` 섹션의 boolean `false`로 override되면 정책 파일을 생성하지 않음
  - `build`: 외부 `codex:` 섹션의 문자열 `"true"`는 정책 파일을 생성하지 않음
  - `build`: source extra `agents/openai.yaml`이 있는 Skill은 metadata trigger 여부와 무관하게 source extra 내용 복사 확인
  - `build --clean`: clean 후에도 정책 파일 재생성 확인
  - `sync`: source에 없어서 생성된 Skill 정책 파일이 source로 추가되지 않음
  - `sync`: source에 있던 Skill `agents/openai.yaml`은 target 수정과 삭제가 source로 반영됨

## Acceptance Criteria

- [x] Codex Command의 최종 metadata에 `disable-model-invocation: true`가 있으면 `.agents/skills/[name]/agents/openai.yaml`이 생성된다.
- [x] Codex Skill의 최종 metadata에 `disable-model-invocation: true`가 있고 source extra `agents/openai.yaml`이 없으면 `.agents/skills/[name]/agents/openai.yaml`이 생성된다.
- [x] Codex Skill의 source extra `agents/openai.yaml`이 있으면 생성 파일로 덮지 않고 source extra를 복사한다.
- [x] 생성된 Skill 정책 파일은 `atb sync` 때 source로 추가되지 않는다.
- [x] source에 원래 있던 Skill `agents/openai.yaml`은 `atb sync` 때 수정과 삭제가 반영된다.
- [x] Codex 빌드 대상 Skill의 source `agents/openai.yaml`이 `exclude`에 걸리면 명확한 오류와 함께 build/sync가 중단된다.
- [x] 선택되지 않은 Skill의 source `agents/openai.yaml`이 `exclude`에 걸려도 build/sync가 중단되지 않는다.
- [x] 외부 `codex:` 메타데이터와 override 결과를 포함한 최종 metadata 기준으로 정책 파일 생성 여부가 결정된다.
- [x] `disable-model-invocation`이 boolean `true`가 아니면 정책 파일을 생성하지 않는다.
- [x] `cargo test`가 모두 통과한다.
- [x] `cargo clippy --all-targets -- -D warnings`가 오류 없이 통과한다.
