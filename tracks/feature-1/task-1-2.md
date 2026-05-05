# Task 1.2: Sync 제외 처리 및 E2E 검증

## Overview

Task 1.1에서 생성된 Skill `agents/openai.yaml`은 build 산출물이므로 source에 자동 추가되면 안 됩니다. 반면 source에 원래 존재하던 Skill `agents/openai.yaml`은 기존 extra 파일처럼 sync되어야 합니다.

이 작업에서는 source 정책 파일이 exclude에 걸린 경우를 `AppContext`에서 fail-fast로 검증하고, transformer가 생성 extra ignore path를 제공할 수 있도록 hook을 추가합니다. 이후 `ExtraSyncer`가 특정 relative path를 Add/Update/Delete 계산에서 제외할 수 있게 하고, Codex E2E 테스트로 최종 metadata 병합, `build`, `build --clean`, `sync` 시나리오를 함께 검증합니다.

## File Scope

### Target Files

| 파일 | 설명 |
| --- | --- |
| `src/app/context.rs` | 수정 — Codex 빌드 대상 Skill의 source `agents/openai.yaml`이 exclude에 걸리는 경우 fail-fast 검증 및 테스트 추가 |
| `src/transformer/mod.rs` | 수정 — 생성 extra ignore hook 기본 메서드 추가 |
| `src/transformer/codex.rs` | 수정 — 생성된 Skill 정책 파일 ignore path 반환 및 관련 테스트 추가 |
| `src/syncer/extra.rs` | 수정 — ignore path를 고려한 extra sync action 계산 |
| `src/syncer/mod.rs` | 수정 — Skill extra sync 시 transformer ignore path 전달 |
| `tests/e2e_codex_openai_policy_test.rs` | 생성 — Codex build, 최종 metadata 병합, build --clean, sync 정책 파일 동작 검증 |
| `src/syncer/README.md` | 수정 — 생성 파일 ignore 정책 설명 추가 |
| `specs/spec.md` | 수정 — Codex sync 정책에 생성 정책 파일 예외 규칙 추가 |

### Reference Files

| 파일 | 설명 |
| --- | --- |
| `src/app/context.rs` | registry에 등록된 빌드 대상 리소스 기준으로 검증을 수행할 위치 확인 |
| `src/core/filter.rs` | `exclude` 패턴이 source root 기준 상대 경로에 적용되는 방식 확인 |
| `src/loader/mod.rs` | exclude가 scan 단계에서 extras 목록에 영향을 주는 흐름 확인 |
| `src/loader/merger.rs` | 최종 metadata 병합 우선순위와 외부 `codex:` 섹션 override 흐름 확인 |
| `src/syncer/mod.rs` | 현재 `Syncer::sync_resource()`와 Skill extra sync 호출부 확인 |
| `src/syncer/extra.rs` | `ExtraSyncer::sync()`, `check_actions()`, Add/Update/Delete 판정 확인 |
| `src/transformer/mod.rs` | `Transformer` trait 기본 hook 추가 위치 확인 |
| `src/transformer/codex.rs` | Task 1.1에서 추가한 정책 파일 판정 helper 재사용 |
| `tests/e2e_codex_sync_test.rs` | Codex sync fixture와 `.codex/toolkit.yaml` 구조 참고 |
| `tests/e2e_skill_extras_test.rs` | Skill extra copy와 clean 동작 테스트 스타일 참고 |
| `src/syncer/README.md` | ExtraSyncer 설명 갱신 위치 확인 |
| `specs/spec.md` | Codex 변환·검증·동기화 규격 갱신 위치 확인 |

## Workflow

### Step 1: source policy exclude 충돌 검증 추가 (`src/app/context.rs`)

Codex 타겟에서 빌드 대상 Skill의 source 디렉터리에 `agents/openai.yaml`이 실제로 존재하지만 `exclude` 패턴 때문에 loader의 extras 목록에서 빠지는 경우, transformer는 source 정책 파일이 없다고 오인할 수 있습니다. 이 상황은 build/sync 시작 전에 fail-fast로 처리합니다.

검증은 `AppContext::init()`에서 registry 구성이 끝나고 missing resource 검증이 끝난 뒤 수행합니다. registry에는 `toolkit.yaml`에 명시된 리소스만 등록되므로, 선택되지 않은 Skill 때문에 실패하지 않습니다.

```rust
/// Codex 빌드 대상 Skill의 source `agents/openai.yaml`이 `exclude`에 의해
/// source extra 목록에서 누락되는 충돌을 검증한다.
/// 충돌이 있으면 source 정책 파일을 build 산출물로 오인하지 않도록 즉시 오류를 반환한다.
fn validate_codex_openai_policy_not_excluded(
    registry: &LoaderRegistry,
    source_dir: &Path,
    exclude_patterns: &[Pattern],
) -> anyhow::Result<()> {
    for resource in registry.all_resources() {
        let Resource::Skill(skill) = resource else {
            continue;
        };

        let policy_path = skill
            .base
            .source_path
            .join(CODEX_OPENAI_POLICY_RELATIVE_PATH);

        if !policy_path.is_file() {
            continue;
        }

        let relative_path = policy_path.strip_prefix(source_dir).unwrap_or(&policy_path);
        if let Some(pattern) = exclude_patterns
            .iter()
            .find(|pattern| pattern.matches_path(relative_path))
        {
            anyhow::bail!(
                "Codex skill '{}:{}' has source agents/openai.yaml, but it is excluded by pattern '{}': {}",
                skill.base.plugin,
                skill.base.name,
                pattern.as_str(),
                relative_path.display()
            );
        }
    }

    Ok(())
}
```

`FileFilter::is_valid()`를 재사용하지 않고 `exclude_patterns`만 직접 확인합니다. 이 검증은 hidden file, forbidden file, 파일 유효성 판정이 아니라 "source 정책 파일이 존재하지만 exclude로 인해 source extra로 로드되지 않는 충돌"만 잡는 목적이기 때문입니다.

호출부는 Codex 타겟으로 한정합니다.

```rust
if cfg.target == BuildTarget::Codex {
    Self::validate_codex_openai_policy_not_excluded(&registry, &source_dir, &exclude_patterns)?;
}
```

### Step 2: Transformer ignore hook 추가 (`src/transformer/mod.rs`)

기본 구현은 빈 목록을 반환해야 합니다. 이렇게 하면 Codex 외 타겟의 sync 동작이 바뀌지 않습니다.

```rust
pub trait Transformer {
    // ...기존 메서드...

    /// 빌드가 생성한 extra-like 파일 중 source로 역동기화하지 않을 상대 경로 목록을 반환합니다.
    fn generated_extra_ignore_paths(&self, _resource: &Resource) -> Vec<PathBuf> {
        Vec::new()
    }
}
```

반환 경로는 Skill target 디렉터리 기준 relative path입니다. 이번 기능에서는 `agents/openai.yaml`만 반환합니다.

### Step 3: Codex ignore hook 구현 (`src/transformer/codex.rs`)

Codex Skill에서 source extra 없이 생성된 정책 파일만 ignore path로 반환합니다. source extra가 있으면 빈 목록을 반환하여 기존 sync가 수행되도록 합니다.

```rust
/// 빌드가 생성한 extra-like 파일 중 source로 역동기화하지 않을 상대 경로를 반환한다.
fn generated_extra_ignore_paths(&self, resource: &Resource) -> Vec<PathBuf> {
    let Resource::Skill(skill) = resource else {
        return Vec::new();
    };

    if requires_openai_policy(&skill.base.metadata) && !has_source_openai_policy(skill) {
        return vec![PathBuf::from(CODEX_OPENAI_POLICY_RELATIVE_PATH)];
    }

    Vec::new()
}
```

이 hook은 Command에는 적용하지 않습니다. Command sync는 Skill extra sync를 수행하지 않기 때문입니다.

### Step 4: ExtraSyncer ignore path 지원 (`src/syncer/extra.rs`)

기존 `sync()` API는 유지하고, ignore path를 받는 새 메서드를 추가합니다.

```rust
/// ignore path 없이 기존 extra sync 동작을 수행한다.
pub fn sync(&self, source_dir: &Path, target_dir: &Path) -> Result<()> {
    self.sync_with_ignored_paths(source_dir, target_dir, &[])
}

/// 지정한 relative path를 제외하고 source와 target extra 파일을 동기화한다.
pub fn sync_with_ignored_paths(
    &self,
    source_dir: &Path,
    target_dir: &Path,
    ignored_paths: &[PathBuf],
) -> Result<()> {
    for action in self.check_actions(source_dir, target_dir, ignored_paths)? {
        // ...기존 action 실행 로직...
    }
    Ok(())
}
```

`check_add_or_update()`와 `check_delete()`는 relative path가 ignore 목록에 있으면 즉시 `Ok(None)`을 반환합니다. 해시 계산 전에 제외해야 불필요한 파일 읽기를 피할 수 있습니다.

```rust
/// relative path가 sync 제외 목록에 포함되는지 확인한다.
fn is_ignored(relative_path: &Path, ignored_paths: &[PathBuf]) -> bool {
    ignored_paths.iter().any(|ignored| ignored == relative_path)
}
```

### Step 5: Syncer에서 ignore path 전달 (`src/syncer/mod.rs`)

Skill extra sync 호출부에서 transformer hook을 호출하여 `ExtraSyncer`에 전달합니다.

```rust
if let Resource::Skill(s) = resource {
    let target_skill_dir = target_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of {:?}", target_path))?;

    let ignored_paths = transformer.generated_extra_ignore_paths(resource);
    self.extra
        .sync_with_ignored_paths(&s.base.source_path, target_skill_dir, &ignored_paths)?;
}
```

Codex 외 transformer는 빈 목록을 반환하므로 기존 동작이 유지됩니다.

### Step 6: AppContext exclude 검증 테스트 추가 (`src/app/context.rs`)

빌드 대상 여부에 따라 검증 결과가 달라지는지 확인합니다.

```rust
/// Codex 빌드 대상 Skill의 source agents/openai.yaml이 exclude에 걸리면 초기화가 실패한다.
#[test]
fn test_init_fails_when_selected_codex_skill_policy_file_is_excluded() {
    // source/plugin_a/skills/selected_skill/SKILL.md 생성
    // source/plugin_a/skills/selected_skill/agents/openai.yaml 생성
    // .codex/toolkit.yaml에 selected_skill 등록
    // exclude: ["**/agents/openai.yaml"]
    // AppContext::init() 에러 메시지에 skill 이름, relative path, exclude pattern 포함 확인
}

/// 선택되지 않은 Skill의 source agents/openai.yaml이 exclude에 걸려도 초기화는 실패하지 않는다.
#[test]
fn test_init_ignores_excluded_policy_file_for_unselected_codex_skill() {
    // source/plugin_a/skills/selected_skill/SKILL.md 생성
    // source/plugin_a/skills/unselected_skill/SKILL.md 생성
    // source/plugin_a/skills/unselected_skill/agents/openai.yaml 생성
    // .codex/toolkit.yaml에는 selected_skill만 등록
    // exclude: ["**/agents/openai.yaml"]
    // AppContext::init() 성공 확인
}
```

### Step 7: ExtraSyncer 단위 테스트 추가 (`src/syncer/extra.rs`)

ignore path가 Add/Update/Delete 판정에서 제외되는지 확인합니다. source에 원래 존재하는 파일을 sync해야 하는 케이스는 ignore 목록을 비워 검증합니다.

```rust
/// ignore path에 포함된 target 파일은 source로 추가하지 않는다.
#[test]
fn test_planner_ignores_generated_extra_add() -> Result<()> {
    // target_dir/agents/openai.yaml만 존재
    // ignored_paths = ["agents/openai.yaml"]
    // actions에 Add가 없는지 확인
    // ...
}

/// ignore path가 비어 있으면 기존 source extra 업데이트가 유지된다.
#[test]
fn test_planner_updates_source_extra_when_not_ignored() -> Result<()> {
    // source_dir/agents/openai.yaml과 target_dir/agents/openai.yaml을 서로 다른 내용으로 생성
    // ignored_paths = []
    // actions에 Update가 있는지 확인
    // ...
}

/// ignore path에 포함된 source와 target 파일 내용이 달라도 Update 액션을 만들지 않는다.
#[test]
fn test_planner_ignores_generated_extra_update() -> Result<()> {
    // source_dir/agents/openai.yaml과 target_dir/agents/openai.yaml을 서로 다른 내용으로 생성
    // ignored_paths = ["agents/openai.yaml"]
    // actions에 Update가 없는지 확인
    // ...
}

/// ignore path에 포함된 source 파일은 target에 없어도 삭제하지 않는다.
#[test]
fn test_planner_ignores_generated_extra_delete() -> Result<()> {
    // source_dir/agents/openai.yaml만 생성하고 target_dir에는 같은 파일을 만들지 않음
    // ignored_paths = ["agents/openai.yaml"]
    // actions에 Delete가 없는지 확인
    // ...
}

/// ignore path가 비어 있으면 target에서 삭제된 source extra도 삭제 대상으로 유지한다.
#[test]
fn test_planner_deletes_source_extra_when_not_ignored() -> Result<()> {
    // source_dir/agents/openai.yaml만 생성하고 target_dir에는 같은 파일을 만들지 않음
    // ignored_paths = []
    // actions에 Delete가 포함되는지 확인
    // ...
}
```

### Step 8: Codex E2E 테스트 추가 (`tests/e2e_codex_openai_policy_test.rs`)

새 E2E 파일을 만들어 build와 sync 시나리오를 한 곳에서 검증합니다.

필수 fixture:

- Command `policy_cmd`
  - Frontmatter 또는 `codex:` 외부 메타데이터에 `disable-model-invocation: true`
- Command `external_policy_cmd`
  - Frontmatter에는 trigger가 없고 외부 `external_policy_cmd.yaml`의 `codex:` 섹션에 `disable-model-invocation: true`
- Skill `generated_policy_skill`
  - `SKILL.md`에 `disable-model-invocation: true`
  - source `agents/openai.yaml` 없음
- Skill `external_false_override_skill`
  - `SKILL.md` Frontmatter에는 `disable-model-invocation: true`
  - `SKILL.yaml`의 `codex:` 섹션에는 `disable-model-invocation: false`
  - source `agents/openai.yaml` 없음
- Skill `external_string_policy_skill`
  - `SKILL.yaml`의 `codex:` 섹션에 `disable-model-invocation: "true"`
  - source `agents/openai.yaml` 없음
- Skill `source_policy_skill`
  - `SKILL.md`에 `disable-model-invocation: true`
  - source `agents/openai.yaml` 있음
- Skill `source_policy_without_trigger_skill`
  - `disable-model-invocation` 누락 또는 boolean `false`
  - source `agents/openai.yaml` 있음

검증 시나리오:

- `atb build`
  - `.agents/skills/policy_cmd/agents/openai.yaml` 생성
  - `.agents/skills/external_policy_cmd/agents/openai.yaml` 생성
  - `.agents/skills/generated_policy_skill/agents/openai.yaml` 생성
  - `.agents/skills/external_false_override_skill/agents/openai.yaml`은 생성되지 않음
  - `.agents/skills/external_string_policy_skill/agents/openai.yaml`은 생성되지 않음
  - `.agents/skills/source_policy_skill/agents/openai.yaml`은 source extra 내용과 동일
  - `.agents/skills/source_policy_without_trigger_skill/agents/openai.yaml`도 source extra 내용과 동일
- `atb build --clean`
  - stale policy 파일을 미리 만들어도 clean 후 지정된 정책 파일 내용으로 재생성
- `atb sync`
  - generated policy skill의 source에는 `agents/openai.yaml`이 새로 생기지 않음
  - source policy skill의 target `agents/openai.yaml`을 수정하면 source extra 내용이 수정됨
  - source policy skill의 target `agents/openai.yaml`을 삭제하면 source extra 파일도 삭제됨

### Step 9: Syncer README와 명세 갱신

`src/syncer/README.md`에는 다음 내용을 추가합니다.

- Transformer가 생성 extra ignore path를 제공할 수 있다.
- Codex Skill에서 source에 없어서 생성된 `agents/openai.yaml`은 sync 대상에서 제외된다.
- source에 원래 있던 `agents/openai.yaml`은 ignore되지 않고 기존 extra sync 정책을 따른다.

`specs/spec.md`에는 Codex 정책 파일 생성, exclude 충돌 검증, sync 예외를 추가합니다.

## Success Criteria

- [x] Codex 빌드 대상 Skill의 source `agents/openai.yaml`이 exclude에 걸리면 `AppContext::init()`이 실패한다.
- [x] 선택되지 않은 Skill의 source `agents/openai.yaml`이 exclude에 걸려도 `AppContext::init()`이 실패하지 않는다.
- [x] `Transformer::generated_extra_ignore_paths()` 기본 구현이 추가되어 Codex 외 타겟 동작이 유지된다.
- [x] `CodexTransformer`가 source extra 없이 생성된 Skill `agents/openai.yaml`만 ignore path로 반환한다.
- [x] `ExtraSyncer`가 ignore path에 포함된 target 파일을 source로 추가하지 않는다.
- [x] `ExtraSyncer`가 ignore path에 포함된 target 파일의 Update 액션을 만들지 않는다.
- [x] source에 원래 있던 Skill `agents/openai.yaml`은 ignore되지 않아 수정과 삭제가 source에 반영된다.
- [x] Codex E2E 테스트가 외부 `codex:` 섹션 boolean `true`, 외부 `codex:` 섹션 boolean `false` override, 문자열 `"true"` 케이스를 통해 최종 metadata 기준 판정을 검증한다.
- [x] Codex E2E 테스트가 `build`, `build --clean`, `sync`의 정책 파일 동작을 검증한다.
- [x] `cargo test`가 모두 통과한다.
- [x] `cargo clippy --all-targets -- -D warnings`가 오류 없이 통과한다.
- [x] `src/syncer/README.md`와 `specs/spec.md`가 sync 예외 정책을 설명한다.
