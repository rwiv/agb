# agb (Agents Builder)

`agb`는 다양한 AI 코딩 에이전트(Claude-code, Gemini-cli, OpenCode 등)를 위한 워크플로우 리소스(Commands, Agents, Skills)를 단일 소스에서 관리하고, 각 에이전트의 규격에 맞게 자동으로 변환하여 빌드해주는 CLI 도구입니다.

## 배경 및 목적

현재 여러 코딩 에이전트들은 각기 다른 설정 방식과 파일 구조를 가지고 있습니다. 이로 인해 동일한 프롬프트나 스킬을 여러 에이전트에서 사용하려면 중복된 파일을 생성하고 관리해야 하는 번거로움이 발생합니다. `agb`는 이러한 중복을 제거하고, 한 곳에서 정의한 워크플로우를 여러 에이전트 환경으로 편리하게 배포할 수 있도록 돕습니다.

## 핵심 가치

- **중복 제로 (Zero Redundancy):** 단일 마크다운 소스로 여러 에이전트를 지원합니다.
- **명시적 관리:** `agb.yaml` 설정 파일을 통해 포함할 리소스를 엄격하게 제어합니다.
- **이식성:** 플러그인 구조를 통해 워크플로우를 쉽게 공유하고 재사용할 수 있습니다.

## 주요 기능

- **플러그인 기반 리소스 관리:** `tests/fixtures/plugins/` 디렉터리 내에 독립적인 기능 단위로 Commands, Agents, Skills를 구성합니다.
- **명시적 빌드 제어:** `agb.yaml`에 선언된 리소스만 선택적으로 빌드에 포함합니다.
- **에이전트별 포맷 변환:**
  - **Gemini-cli:** 마크다운 커맨드를 TOML 규격으로 자동 변환합니다.
  - **Claude-code / OpenCode:** 에이전트에 최적화된 마크다운 구조로 빌드합니다.
- **충돌 방지:** 서로 다른 플러그인 간의 리소스 이름 충돌을 감지하여 안전한 빌드를 보장합니다.

## 프로젝트 구조

`agb`는 소스 데이터(Source)와 빌드 결과물(Output)이 위치하는 곳을 분리하여 관리합니다.

- **Source 디렉터리 (리소스 저장소)**: `plugins/`와 `AGENTS.md`가 위치합니다.
- **Output 디렉터리 (에이전트 워크스페이스)**: `agb.yaml`이 위치하며, 빌드 결과물이 생성됩니다.

```text
[Output Workspace]/
└── agb.yaml        # 빌드 설정 파일 (소스 절대 경로 포함)

[Source Repository]/
├── AGENTS.md       # 전역 시스템 프롬프트
└── plugins/        # 리소스 플러그인 저장소
    └── [plugin_name]/
        ├── commands/
        ├── agents/
        └── skills/
```

## 설정 방법 (`agb.yaml`)

에이전트 워크스페이스 루트에 `agb.yaml` 파일을 작성하여 빌드 타겟과 포함할 리소스를 정의합니다.

```yaml
source: /Users/path/to/source_repo # 소스 리소스의 절대 경로 (필수)
target: gemini-cli # 지원: gemini-cli, claude-code, opencode
exclude:
  - "*.kor.md"
  - "*.tmp"
resources:
  commands:
    - plugin_a:foo
    - plugin_b:bar
  agents:
    - plugin_a:researcher
  skills:
    - plugin_c:python_expert
```

## 시작하기 (개발자용)

본 프로젝트는 Rust로 작성되었습니다.

### 빌드 및 실행

```bash
# 빌드
cargo build --release

# 실행 (agb.yaml이 있는 디렉터리에서)
./target/release/agb build
```
