# Resource & Configuration Format

이 문서는 `agb` 시스템에서 사용하는 모든 파일의 물리적 구조, 설정 규격(`agb.yaml`), 그리고 리소스 작성 형식을 정의합니다.

## 1. 소스 리소스 구조 (Source)

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

## 2. 빌드 환경 구조 (Output)

`agb.yaml`이 위치한 디렉터리가 빌드 루트가 되며, 결과물은 해당 위치에 다음과 같은 구조로 생성됩니다.

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

프로젝트 루트에서 빌드 및 동기화 동작을 제어하는 메인 설정 파일입니다.

| 필드 | 설명 | 비고 |
| :--- | :--- | :--- |
| `source` | 소스 리소스 저장소의 경로 | 물결표(`~`) 확장 지원 |
| `target` | 빌드/동기화 대상 에이전트 | `gemini-cli`, `claude-code`, `opencode` |
| `exclude` | 스캔/동기화에서 제외할 패턴 | Glob 패턴 지원 |
| `resources` | 대상 리소스 명시 | `[플러그인명]:[리소스명]` 형식 |

**작성 예시**:
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

## 4. 리소스 작성 상세 규격

### 4.1 메타데이터 작성

- **YAML Frontmatter**: 마크다운 파일 상단에 `---`로 구분하여 작성. 모든 공용 메타데이터(name, description 등)는 여기서 관리합니다.
- **외부 메타데이터**: 마크다운과 동일한 이름의 `.yaml` 또는 `.yml` 파일. **타겟 에이전트 전용 예약어 섹션만 포함할 수 있습니다.**
- **우선순위**: 외부 파일 내의 타겟 전역 예약어 섹션 내용이 Frontmatter의 공용 설정을 최종적으로 오버라이트합니다. 상세 알고리즘은 [design.md](./design.md) 및 [spec.md](./spec.md)를 참조하십시오.

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

플러그인 루트에서 리소스 간의 의존성을 명시적으로 정의합니다. 빌드 시 `agb.yaml`에 포함되지 않은 리소스를 의존하고 있을 경우 빌드가 실패합니다.

**포맷**:
```yaml
[resource_type_plural]:
  [resource_name]:
    [dependency_type_plural]:
      - [plugin_name]:[resource_name]
```

## 5. 추가 파일 (Extra Files)

`Skill` 타입 리소스 폴더 내의 `SKILL.md`와 `SKILL.yaml`을 제외한 모든 파일은 `extras`로 분류되어, 변환 단계 후 빌드 대상 폴더에 물리적으로 복사됩니다. 복사 시 원본 폴더의 하위 디렉터리 구조가 그대로 유지됩니다.
