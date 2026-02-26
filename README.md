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
            ├── commands/   # [name].md + [name].yaml (파일 쌍)
            └── skills/     # [name]/SKILL.yaml + *.md (폴더 구조)
```

## 설정 규격

### 빌드 구성 (`agb.yaml`)

```yaml
source: ~/agb-resources      # 리소스 소스 저장소 경로
target: gemini-cli           # 빌드 타겟 (gemini-cli, claude-code, opencode)
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

**Skill 정의**: 스킬은 캡슐화된 폴더 구조를 가지며, `SKILL.yaml`을 필수 메타데이터로 포함해야 합니다.
- `plugins/my_plugin/skills/analyzer/SKILL.yaml` (메타데이터)
- `plugins/my_plugin/skills/analyzer/prompt.md` (명령어 본문)

## 문서 가이드
상세한 설계 사양 및 기술 규격은 `specs/` 디렉토리를 참조하십시오.

- [**PRD.md**](./specs/PRD.md): 제품 요구사항 및 비즈니스 목표
- [**DESIGN.md**](./specs/DESIGN.md): 시스템 아키텍처 및 내부 설계
- [**SPEC.md**](./specs/SPEC.md): 상세 기술 규격 및 변환 규칙

