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
- **유연한 메타데이터:** JSON(`.json`) 및 YAML(`.yaml`, `.yml`) 형식의 메타데이터 지원.
- **에이전트별 포맷 변환:**
  - **Gemini-cli:** 마크다운 본문을 TOML의 `prompt` 필드로 자동 변환.
  - **Claude-code / OpenCode:** Frontmatter를 포함한 최적화된 마크다운 구조로 빌드.
- **충돌 방지:** 서로 다른 플러그인 간의 리소스 이름 중복을 감지하여 안전한 빌드 보장.

## 프로젝트 구조

### 소스 리소스 (Source)
리소스는 본문(`*.md`)과 메타데이터(`*.json/yaml`)의 조합으로 구성됩니다.

```text
[Source Repository]/
├── AGENTS.md               # 전역 시스템 지침 (타겟에 따라 GEMINI.md 등으로 변환)
└── plugins/
    └── [plugin_name]/
        ├── commands/       # 파일 쌍: [name].md + [name].json/yaml
        ├── agents/         # 파일 쌍: [name].md + [name].json/yaml
        └── skills/         # 폴더 구조: [skill_name]/METADATA.json + 기타 파일들
```

### 빌드 환경 (Output)
`agb.yaml`이 위치한 곳이 빌드 루트가 되며, 결과물이 해당 위치에 생성됩니다.

```text
[Output Workspace]/
├── agb.yaml                # 빌드 설정 파일
├── GEMINI.md               # 빌드된 전역 지침 (target이 gemini-cli인 경우)
├── commands/               # 변환된 커맨드 파일들
└── ...
```

## 리소스 작성 예시

`agb` 리소스는 마크다운 본문과 메타데이터(JSON 또는 YAML) 파일이 한 쌍으로 구성됩니다.

### Command 예시

`plugins/my_plugin/commands/greet.md`:
```markdown
Hello! How can I help you today?
```

`plugins/my_plugin/commands/greet.yaml`:
```yaml
name: greet
description: A simple greeting command
model: gemini-2.0-flash
```

빌드 시(Gemini-cli 타겟), 위 파일들은 `commands/greet.toml`로 병합됩니다:
```toml
name = "greet"
description = "A simple greeting command"
model = "gemini-2.0-flash"
prompt = "Hello! How can I help you today?"
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

## 개발 환경

- **Language:** Rust (Edition 2024)
- **Main Dependencies:** `clap`, `serde`, `toml`, `walkdir`, `glob`
