# Technical Specification

## 1. 시스템 개요 (System Overview)

`agb`는 여러 플러그인에 분산된 AI 에이전트 리소스(Commands, Agents, Skills)를 수집하고, 타겟 에이전트(Gemini-cli, Claude-code 등)의 규격에 맞게 변환하여 빌드하는 CLI 도구입니다.

## 2. 프로젝트 구조 (Project Structure)

### 2.1 소스 리소스 구조 (Source)
리소스는 필수 본문(`*.md`)과 선택적 메타데이터(`*.yaml`, `*.yml`)의 조합으로 구성됩니다. **본문 파일(*.md)이 누락된 리소스는 빌드 시 에러를 유발합니다.**

```text
[Source Repository]/
├── AGENTS.md               # 전역 시스템 지침 (Frontmatter 지원)
└── plugins/
    └── [plugin_name]/
        ├── commands/       # 필수: [name].md | 선택: [name].yaml
        ├── agents/         # 필수: [name].md | 선택: [name].yaml
        └── skills/         # 필수: [skill_name]/SKILL.md | 선택: SKILL.yaml
                            # 추가 파일(예: 스크립트, 데이터 등)은 하위 디렉터리 구조를 유지하며 빌드 폴더로 복사됩니다.
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
        ├── SKILL.md        # 변환된 스킬 본문
        └── extra_file.py   # 소스 폴더에서 복사된 추가 파일들
```

## 3. 설정 규격 (`agb.yaml`)

| 필드 | 설명 | 비고 |
| :--- | :--- | :--- |
| `source` | 소스 리소스 저장소의 경로 | 물결표(`~`) 확장 지원 |
| `target` | 빌드/동기화 대상 에이전트 | `gemini-cli`, `claude-code`, `opencode` |
| `exclude` | 스캔/동기화에서 제외할 패턴 | Glob 패턴 지원 |
| `resources` | 대상 리소스 명시 | `[플러그인명]:[리소스명]` 형식 |

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
`loader::ResourceParser`가 빌드 타겟에 따라 다음과 같은 순서로 메타데이터를 처리합니다 (Shallow Merge):
1.  **Extract Base**: `.md` 파일에서 YAML Frontmatter 추출.
2.  **External Process (Optional)**: 외부 메타데이터 파일(`.yaml`/`.yml`)이 존재하는 경우에만 다음을 수행:
    - **Validate**: 최상위 키가 모두 타겟 예약어(`gemini-cli`, `claude-code`, `opencode`)인지 검증. 일반 필드 발견 시 빌드 오류.
    - **Override**: 현재 빌드 타겟(`BuildTarget`) 섹션 내용을 Base에 덮어씀.
    - **Cleanup**: 결과물에서 모든 타겟 예약어 키들을 제거.
3.  **Finalize**: 리소스 이름은 원본 파일명을 그대로 사용하며, 병합된 메타데이터와 본문을 결합하여 리소스 객체 완성.

### 5.2 보안 및 제약 사항
- **타겟 전용 파일 금지**: 플러그인 내부에는 `GEMINI.md`, `CLAUDE.md`, `OPENCODE.md`와 같은 파일이 존재할 수 없습니다. 발견 시 빌드가 중단됩니다.
- **충돌 검사 (Conflict Check)**: 서로 다른 플러그인에서 동일한 **타입**과 **이름**을 가진 리소스가 동시에 빌드 대상으로 선택된 경우 빌드를 실패 처리합니다. 리소스의 타입이 다르면 동일한 이름을 가질 수 있습니다 (예: `command/write-plan`과 `skill/write-plan`은 공존 가능).

## 6. 타겟별 변환 사양 (Transformation)

- **Gemini-cli**: 
  - `commands/[name].toml` 생성. 마크다운 본문은 `prompt` 필드로, 메타데이터는 최상위 키로 매핑.
  - `agents/[name].md`, `skills/[name]/SKILL.md` 생성. 메타데이터를 포함한 마크다운 구조(`DefaultTransformer`)로 빌드.
  - `AGENTS.md` -> `GEMINI.md` 변환. (변환 전 Frontmatter는 제거됨)
- **Claude-code / OpenCode**: 
  - `commands/[name].md`, `agents/[name].md`, `skills/[name]/SKILL.md` 생성. (`DefaultTransformer` 사용)
  - 메타데이터를 YAML Frontmatter로, 본문을 마크다운 내용으로 결합.
  - `AGENTS.md` -> `CLAUDE.md` 또는 `OPENCODE.md` 변환. (변환 전 원본 Frontmatter는 제거됨)

## 8. 동기화 규격 (Sync Specifications)

`agb sync` 실행 시 다음과 같은 규칙에 따라 소스를 업데이트합니다.

### 8.1. 수술적 업데이트 (Surgical Update)
- **대상**: 마크다운(`.md`) 파일의 `description` 필드.
- **규칙**: 원본 파일의 주석이나 들여쓰기를 보존하기 위해, YAML 파싱 대신 정규표현식을 사용한 라인 단위 교체를 수행합니다.

### 8.2. 본문 동기화 (Content Sync)
- **대상**: 리소스의 마크다운 본문.
- **규칙**: Frontmatter 영역을 제외한 순수 본문 전체를 타겟의 내용으로 교체합니다.

### 8.3. 스킬 디렉터리 동기화 (Full Skill Sync)
- **추가**: 타겟에 새로 생성된 파일을 소스로 복사 (단, `exclude` 대상 제외).
- **삭제**: 타겟에서 사라진 파일을 소스에서 제거 (단, `SKILL.md` 및 `exclude` 대상 제외).
- **수정**: 파일 해시(SHA-256) 비교 후 변경 시 덮어쓰기.

## 9. 예외 처리 전략

- `agb.yaml` 미존재: 명확한 가이드 메시지와 함께 종료.
- 경로 확장 실패: 시스템 홈 디렉터리 접근 불가 시 에러 처리.
- Clean 단계 실패: 권한 부족 등으로 기존 파일 삭제 불가 시 안전을 위해 빌드 중단.
