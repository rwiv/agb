# agb (Agents Builder)

`agb`는 다양한 AI 코딩 에이전트(Claude-code, Gemini-cli, OpenCode 등)를 위한 워크플로우 리소스(Commands, Agents, Skills)를 단일 소스에서 관리하고, 각 에이전트의 규격에 맞게 자동으로 변환하여 빌드해주는 CLI 도구입니다.

## 배경 및 목적

여러 코딩 에이전트는 각기 다른 설정 방식과 파일 구조를 가집니다. 이로 인해 동일한 프롬프트나 스킬을 여러 에이전트에서 사용하려면 중복된 파일을 관리해야 합니다. `agb`는 이러한 중복을 제거하고, 한 곳에서 정의한 워크플로우를 여러 에이전트 환경으로 편리하게 배포할 수 있도록 돕습니다.

## 핵심 가치

- **중복 제로 (Zero Redundancy):** 단일 마크다운 소스로 여러 에이전트 지원.
- **명시적 관리:** `agb.yaml`을 통한 리소스 포함 여부의 엄격한 제어.
- **이식성:** 플러그인 구조를 통한 워크플로우 공유 및 재사용 용이.

## 주요 기능

- **플러그인 기반 리소스 관리:** `plugins/` 내에 독립적인 기능 단위로 리소스 구성.
- **유연한 메타데이터:** YAML(`.yaml`, `.yml`) 형식의 외부 메타데이터 파일 지원뿐만 아니라, 마크다운 파일 내의 **YAML Frontmatter**도 지원합니다.
- **타겟 기반 오버라이트:** 외부 메타데이터 파일 내에 에이전트별 섹션(`gemini`, `claude`, `opencode`)을 두어 빌드 타겟에 따라 설정을 동적으로 변경할 수 있습니다.
- **에이전트별 포맷 변환:**
  - **Gemini-cli:** Commands 리소스는 TOML로 자동 변환하며, Agents와 Skills는 메타데이터가 포함된 마크다운 구조(`SKILL.md`)로 빌드합니다.
  - **Claude-code / OpenCode:** Frontmatter를 포함한 최적화된 마크다운 구조로 빌드합니다.
- **충돌 방지:** 서로 다른 플러그인 간의 리소스 이름 중복을 감지하여 안전한 빌드 보장.

## 프로젝트 구조

### 소스 리소스 (Source)
리소스는 본문(`*.md`)과 메타데이터(`*.yaml`, `*.yml`)의 조합으로 구성됩니다. 마크다운 파일 내에 Frontmatter를 직접 포함할 수도 있습니다.

```text
[Source Repository]/
├── AGENTS.md               # 전역 시스템 지침 (타겟에 따라 GEMINI.md 등으로 변환, Frontmatter 지원)
└── plugins/
    └── [plugin_name]/
        ├── commands/       # 파일 쌍: [name].md (FM 포함 가능) + [name].yaml/yml (선택 사항)
        ├── agents/         # 파일 쌍: [name].md (FM 포함 가능) + [name].yaml/yml (선택 사항)
        └── skills/         # 폴더 구조: [skill_name]/SKILL.yaml + 기타 파일들
```

### 빌드 환경 (Output)
`agb.yaml`이 위치한 곳이 빌드 루트가 되며, 결과물이 해당 위치에 생성됩니다.

```text
[Output Workspace]/
├── agb.yaml                # 빌드 설정 파일
├── GEMINI.md               # 빌드된 전역 지침 (target이 gemini-cli인 경우, FM 제거됨)
├── commands/               # 변환된 커맨드 파일들
├── agents/                 # 변환된 에이전트 파일들
└── skills/                 # 변환된 스킬 폴더들
    └── [skill_name]/
        └── SKILL.md        # 변환된 스킬 본문
```

## 리소스 작성 예시

### Frontmatter 및 타겟 오버라이트 예시

`plugins/my_plugin/agents/researcher.md`:
```markdown
---
name: researcher
model: default-model
---
You are a professional researcher. Analyze the given topic deeply.
```

`plugins/my_plugin/agents/researcher.yaml` (외부 메타데이터):
```yaml
gemini:
  model: gemini-3.0-pro
claude:
  model: claude-3-opus
```

빌드 시(Gemini-cli 타겟), 외부 파일의 `gemini` 섹션이 Frontmatter의 기본 설정을 오버라이트하여 `agents/researcher.md`가 생성됩니다:
```markdown
---
metadata:
  name: researcher
  model: gemini-3.0-pro
---

You are a professional researcher. Analyze the given topic deeply.
```

## 설정 방법 (`agb.yaml`)

```yaml
source: ~/projects/agb-resources # 소스 리소스 경로 (물결표 '~' 지원)
target: gemini-cli               # 지원: gemini-cli, claude-code, opencode
exclude:
  - "*.kor.md"
  - "*.tmp"
resources:
  commands:
    - plugin_a:foo               # [플러그인명]:[리소스명] 형식
  skills:
    - plugin_c:python_expert
```

## 시작하기

### 설치 (Rust 환경 필요)

```bash
cargo install --path .
```

### 빌드 실행

`agb.yaml` 파일이 있는 디렉터리에서 실행합니다.

```bash
# 기본 설정(agb.yaml)으로 빌드
agb build

# 특정 설정 파일 지정
agb build --config custom-agb.yaml
```
