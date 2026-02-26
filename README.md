# agb (Agents Builder)

`agb`는 다양한 AI 코딩 에이전트를 위한 워크플로우 리소스(Commands, Agents, Skills)를 단일 소스에서 관리하고 빌드하는 CLI 도구입니다.

## 핵심 가치
- **중복 제로**: 단일 마크다운 소스로 여러 에이전트 지원.
- **명시적 관리**: `agb.yaml`을 통한 엄격한 리소스 제어.
- **이식성**: 플러그인 구조를 통한 쉬운 공유 및 재사용.

## 시작하기

### 설치 (Rust 환경 필요)
```bash
cargo install --path .
```

## 사용 가이드

### 빌드 워크플로우
1. 프로젝트 루트에 `agb.yaml` 설정 파일을 생성합니다.
2. `agb build` 명령을 실행하여 빌드 파이프라인을 가동합니다.
3. 타겟 에이전트 규격에 따라 생성된 `commands/`, `agents/`, `skills/` 결과를 확인합니다.

### 동기화 워크플로우
1. 타겟 에이전트 환경에서 프롬프트나 스킬 파일을 수정합니다.
2. `agb sync` 명령을 실행하여 변경사항을 원본 소스로 동기화합니다.
3. 소스 마크다운 파일의 `description`과 본문이 업데이트된 것을 확인합니다.

## 프로젝트 구조

```text
.
├── agb.yaml                # 빌드 구성 정의 (Config)
├── GEMINI.md               # (Output) 변환된 전역 시스템 지침
├── commands/               # (Output) 타겟별 커맨드 리소스
└── [Source Directory]/     # 소스 리소스 저장소 (plugins 포함)
    ├── AGENTS.md           # 전역 시스템 지침 원본
    └── plugins/
        └── my_plugin/
            ├── commands/   # [name].md (필수) + [name].yaml (선택)
            └── skills/     # [name]/SKILL.md (필수) + SKILL.yaml (선택)
```

## 설정 규격

### 빌드 구성 (`agb.yaml`)

```yaml
source: ~/agb-resources      # 리소스 소스 저장소 경로
target: gemini-cli           # 빌드 타겟 (gemini-cli, claude-code, opencode)
exclude:
  - "*.tmp"                  # (선택) 스캔/동기화에서 제외할 Glob 패턴
resources:
  commands:
    - my_plugin:web_search   # [플러그인]:[리소스명]
  skills:
    - shared:python_expert
```

### 리소스 정의 예시

**Command 정의** (`plugins/my_plugin/commands/web_search.md`):
```markdown
---
description: Search the web to retrieve the latest information.
---
Search the following query on Google and summarize the results: {{query}}
```

**Skill 정의**: 스킬은 캡슐화된 폴더 구조를 가집니다. 메타데이터(`SKILL.yaml`)는 선택 사항이며, **타겟 에이전트 전용 섹션만 포함할 수 있습니다.**
- `plugins/my_plugin/skills/analyzer/SKILL.md` (명령어 본문 및 공용 메타데이터)
- `plugins/my_plugin/skills/analyzer/SKILL.yaml` (타겟별 오버라이드 설정)

### 리소스 메타데이터 규칙
- **공용 필드**: `name`, `description` 등 모든 공통 설정은 반드시 `.md` 파일의 Frontmatter에 작성해야 합니다.
- **타겟 필드**: 에이전트별 특화 설정은 외부 `.yaml` 파일 내의 타겟 예약어 섹션(`gemini-cli` 등)에 작성합니다. 외부 파일에 일반 필드가 있으면 빌드 시 오류가 발생합니다.

## 문서 가이드
상세한 설계 사양 및 기술 규격은 `specs/` 디렉토리를 참조하십시오.

- [**PRD.md**](./specs/PRD.md): 제품 요구사항 및 비즈니스 목표
- [**DESIGN.md**](./specs/DESIGN.md): 시스템 아키텍처 및 내부 설계
- [**SPEC.md**](./specs/SPEC.md): 상세 기술 규격 및 변환 규칙

