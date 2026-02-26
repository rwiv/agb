# Technical Specification

## 1. 시스템 개요 (System Overview)

`agb`는 여러 플러그인에 분산된 AI 에이전트 리소스(Commands, Agents, Skills)를 수집하고, 타겟 에이전트(Gemini-cli, Claude-code 등)의 규격에 맞게 변환하여 빌드하는 CLI 도구입니다.

## 2. 프로젝트 구조 (Project Structure)

### 2.1 소스 리소스 구조 (Source)
리소스는 본문(`*.md`)과 메타데이터(`*.yaml`, `*.yml`)의 조합으로 구성됩니다.

```text
[Source Repository]/
├── AGENTS.md               # 전역 시스템 지침 (Frontmatter 지원)
└── plugins/
    └── [plugin_name]/
        ├── commands/       # 파일 쌍: [name].md + [name].yaml (선택 사항)
        ├── agents/         # 파일 쌍: [name].md + [name].yaml (선택 사항)
        └── skills/         # 폴더 구조: [skill_name]/SKILL.md + SKILL.yaml (선택 사항)
```

### 2.2 빌드 환경 구조 (Output)
`agb.yaml`이 위치한 디렉터리가 빌드 루트가 되며, 결과물이 해당 위치에 생성됩니다.

```text
[Output Workspace]/
├── agb.yaml                # 빌드 설정 파일
├── GEMINI.md               # 변환된 전역 지침 (타겟에 따라 이름 변경)
├── commands/               # 변환된 커맨드 파일들
├── agents/                 # 변환된 에이전트 파일들
└── skills/                 # 변환된 스킬 폴더들
    └── [skill_name]/
        └── SKILL.md        # 변환된 스킬 본문
```

## 3. 설정 규격 (`agb.yaml`)

| 필드 | 설명 | 비고 |
| :--- | :--- | :--- |
| `source` | 소스 리소스 저장소의 경로 | 물결표(`~`) 확장 지원 |
| `target` | 빌드 대상 에이전트 | `gemini-cli`, `claude-code`, `opencode` |
| `exclude` | 스캔에서 제외할 파일 패턴 목록 | Glob 패턴 지원 |
| `resources` | 빌드에 포함할 리소스 명시 | `[플러그인명]:[리소스명]` 형식 |

**예시**:
```yaml
source: ~/projects/agb-resources
target: gemini-cli
exclude:
  - "*.tmp"
resources:
  commands:
    - my_plugin:web_search
  skills:
    - shared_plugin:python_expert
```

## 4. 리소스 작성 규격

### 4.1 메타데이터 작성
- **YAML Frontmatter**: 마크다운 파일 상단에 `---`로 구분하여 작성. 모든 공용 메타데이터(name, description 등)는 여기서 관리합니다.
- **외부 메타데이터**: 마크다운과 동일한 이름의 `.yaml` 또는 `.yml` 파일. **타겟 에이전트 전용 예약어 섹션만 포함할 수 있습니다.**
- **우선순위**: 외부 파일 내의 타겟 전역 예약어 섹션(`gemini-cli`, `claude-code`, `opencode`) 내용이 Frontmatter의 공용 설정을 최종적으로 오버라이트합니다.

### 4.2 작성 예시 (Agents)
`plugins/my_plugin/agents/researcher.md`:
```markdown
---
name: researcher
model: default-model
---
You are a professional researcher.
```

`plugins/my_plugin/agents/researcher.yaml`:
```yaml
# 타겟 전용 섹션만 허용됩니다. 일반 필드(name 등)는 무시되거나 빌드 에러를 유발합니다.
gemini-cli:
  model: gemini-3.0-pro
```

## 5. 리소스 처리 규격 (Processing Rules)

### 5.1 메타데이터 병합 알고리즘 (Metadata Merge)
`loader::ResourceParser`가 빌드 타겟에 따라 다음과 같은 순서로 메타데이터를 병합합니다 (Shallow Merge):
1.  **Base**: `.md` 파일의 YAML Frontmatter 추출.
2.  **Validate External**: 외부 메타데이터 파일(`.yaml`/`.yml`)을 읽어, 최상위 키가 모두 타겟 예약어(`gemini-cli`, `claude-code`, `opencode`)인지 검증합니다. 예약어가 아닌 일반 필드가 발견되면 빌드 오류를 발생시킵니다.
3.  **Target Override**: 외부 파일 내 현재 빌드 타겟(`BuildTarget`)에 해당하는 섹션 내용을 Base에 덮어씀.
4.  **Cleanup**: 모든 타겟 예약어 키들을 최종 메타데이터 결과물에서 제거.

### 5.2 보안 및 제약 사항
- **타겟 전용 파일 금지**: 플러그인 내부에는 `GEMINI.md`, `CLAUDE.md`, `OPENCODE.md`와 같은 파일이 존재할 수 없습니다. 발견 시 빌드가 중단됩니다.
- **충돌 검사**: 서로 다른 플러그인에서 동일한 이름의 리소스가 선택되어 결과물 디렉터리에서 충돌하는 경우 빌드를 실패 처리합니다.

## 6. 타겟별 변환 사양 (Transformation)

- **Gemini-cli**: 
  - `commands/[name].toml` 생성. 마크다운 본문은 `prompt` 필드로, 메타데이터는 최상위 키로 매핑.
  - `agents/[name].md`, `skills/[name]/SKILL.md` 생성. 메타데이터를 포함한 마크다운 구조(`DefaultTransformer`)로 빌드.
  - `AGENTS.md` -> `GEMINI.md` 변환. (변환 전 Frontmatter는 제거됨)
- **Claude-code / OpenCode**: 
  - `commands/[name].md`, `agents/[name].md`, `skills/[name]/SKILL.md` 생성. (`DefaultTransformer` 사용)
  - 메타데이터를 YAML Frontmatter로, 본문을 마크다운 내용으로 결합.
  - `AGENTS.md` -> `CLAUDE.md` 또는 `OPENCODE.md` 변환. (변환 전 원본 Frontmatter는 제거됨)

## 7. 예외 처리 전략

- `agb.yaml` 미존재: 명확한 가이드 메시지와 함께 종료.
- 경로 확장 실패: 시스템 홈 디렉터리 접근 불가 시 에러 처리.
- Clean 단계 실패: 권한 부족 등으로 기존 파일 삭제 불가 시 안전을 위해 빌드 중단.
