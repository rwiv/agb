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
        ├── deps.yaml       # 선택: 리소스 간 의존성 정의
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

### 4.3 의존성 정의 (`deps.yaml`)
플러그인 루트에 위치한 `deps.yaml`을 통해 리소스 간의 의존성을 명시적으로 정의할 수 있습니다. 빌드 시 `agb.yaml`에 포함되지 않은 리소스를 의존하고 있을 경우 빌드가 실패합니다.

**포맷**:
```yaml
[resource_type_plural]:
  [resource_name]:
    [dependency_type_plural]:
      - [plugin_name]:[resource_name]
```

**예시**:
```yaml
agents:
  researcher:
    skills:
      - shared:web_search
    commands:
      - core:google_search
```

## 5. 리소스 처리 규격 (Processing Rules)

### 5.1 메타데이터 병합 알고리즘 (Metadata Merge)
`loader::merger::MetadataMerger`가 빌드 타겟에 따라 다음과 같은 순서로 메타데이터를 처리합니다 (Shallow Merge):

**병합 우선순위 (Merge Priority)**:
1.  **Markdown Frontmatter (Base)**: 원본 소스 파일의 기본 설정.
2.  **Metadata Map**: `map.yaml`에 정의된 매핑에 따른 변환 값 (문자열 타입 필드 전용, `description` 제외).
3.  **외부 메타데이터 파일 ([name].yaml)**: 타겟 전용 섹션(`gemini-cli` 등)의 명시적 오버라이트.

**병합 단계**:
1.  **Extract Base**: `.md` 파일에서 YAML Frontmatter 추출.
2.  **Apply Mapping (Optional)**: 소스 루트에 `map.yaml`이 존재하는 경우, 정의된 규칙에 따라 필드 값을 타겟에 맞게 치환.정의가 없거나 타겟 값이 없으면 원본 유지.
3.  **External Process (Optional)**: 외부 메타데이터 파일(`.yaml`/`.yml`)이 존재하는 경우에만 다음을 수행:
    - **Validate**: 최상위 키가 모두 타겟 예약어(`gemini-cli`, `claude-code`, `opencode`)인지 검증. 일반 필드 발견 시 빌드 오류.
    - **Override**: 현재 빌드 타겟(`BuildTarget`) 섹션 내용을 이전 결과물에 덮어씀.
4.  **Cleanup**: 최종 결과물에서 모든 타겟 예약어 키들을 제거하여 깨끗한 메타데이터 객체 완성.

### 5.2 의존성 검증 (Dependency Validation)
빌드 파이프라인의 변환(Transform) 단계 직전에 모든 리소스의 의존성을 검증합니다.
- **Fail-fast**: 하나 이상의 의존성이 누락된 경우, 누락된 모든 항목을 리포트하고 빌드를 즉시 중단합니다.
- **비재귀적 검사**: 현재 빌드 대상 리소스가 직접적으로 명시한 의존성만 확인합니다.

### 5.3 보안 및 제약 사항
- **제외 패턴(`exclude`) 적용**: `agb.yaml`에 정의된 패턴은 빌드 스캔 및 동기화 시 모두 적용됩니다. 단, 리소스의 필수 본문(`*.md` 또는 `SKILL.md`)은 제외 패턴에 해당하더라도 시스템 보호를 위해 스캔에서 제외되지 않습니다.
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

`agb sync`는 프로젝트 세션 중에 발생한 리소스의 변경사항을 원본 베이스 디렉토리(Source)로 안전하게 역전파하기 위해 설계되었습니다. 개발자는 프로젝트 환경에서 실시간으로 프롬프트를 튜닝하거나 스킬 파일(extras)을 추가/수정할 수 있으며, `sync` 명령은 이러한 로컬의 변경사항이 중앙의 관리 라이브러리에 누락 없이 반영되도록 보장합니다.

`agb sync` 실행 시 다음과 같은 규칙에 따라 소스를 업데이트합니다.

### 8.1. 부분적 업데이트 (Partial Update)
- **대상**: 마크다운(`.md`) 파일의 `description` 필드.
- **규칙**: 원본 파일의 주석이나 들여쓰기를 보존하기 위해, YAML 파싱 대신 정규표현식을 사용한 라인 단위 교체를 수행합니다.
- **제한 사항**: **멀티라인 `description`은 동기화가 지원되지 않습니다.** 원본 소스에서 YAML 멀티라인 마커(`|`, `>`)가 감지되거나 들여쓰기된 데이터가 발견될 경우, 원본 파손 방지를 위해 동기화 작업이 에러와 함께 중단됩니다.

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
