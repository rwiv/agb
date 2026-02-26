# agb (Agents Builder)

> **"똑똑한 에이전트 워크플로우, 이제 한 곳에서 빌드하고 모든 곳에서 누리세요."**

`agb`는 여러 AI 코딩 에이전트(Claude Code, Gemini Cli 등)의 파편화된 프롬프트와 스킬을 단일 소스에서 관리하고, 각 환경에 맞게 최적화하여 배포하는 **AI 에이전트 리소스 오케스트레이터**입니다.

## 😫 혹시 이런 고통을 겪고 계신가요?

- **설정 복사-붙여넣기의 늪**: 새로운 프로젝트를 만들 때마다 프롬프트와 스킬 파일을 일일이 복사하고 계신가요?
- **흩어진 업데이트**: 프로젝트 세션 중에 수정한 프롬프트, 나중에 다시 쓰려고 보니 어디에 수정했는지 기억이 안 나시나요?
- **에이전트마다 다른 규격**: Claude는 Markdown, Gemini는 TOML... 에이전트를 바꿀 때마다 설정 파일 형식을 바꾸느라 정작 프롬프트 엔지니어링에는 집중하지 못하고 있나요?

## ✨ agb가 당신의 워크플로우를 바꿉니다

`agb`는 **중앙 관리(Base)**와 **자유로운 사용(Project)**의 완벽한 조화를 지향합니다.

### 🏛️ 나만의 워크플로우 (Centralized Library)

프로젝트마다 설정을 파편화하지 마세요. 나만의 '베이스 디렉토리'에 플러그인 단위로 리소스를 모아두고, 필요한 프로젝트에서 쏙쏙 골라 빌드하세요.

### 🔄 양방향 동기화 (Bi-directional Sync)

프로젝트 도중 스킬을 수정하셨나요? `agb sync` 한 번이면 로컬의 변경사항이 중앙 베이스 디렉토리로 안전하게 반영됩니다.

### 🎯 한 번의 정의로 어디서든 사용 (Write Once, Run Everywhere)

단일 마크다운 소스로 작성하세요. `agb`가 타겟 에이전트의 규격(TOML, YAML, JSON 등)에 맞춰 자동으로 변환하여 빌드해 드립니다.

## 🚀 3분 만에 시작하기

### 1. 설치

```bash
cargo install --path .
```

### 2. 프로젝트 설정 (`agb.yaml`)

프로젝트 루트에 `agb.yaml`을 생성하여 베이스 디렉토리와 타겟 에이전트를 지정합니다. 이 설정은 빌드와 동기화 모두의 기준이 됩니다.

```yaml
source: ~/agb-resources      # 📚 리소스 소스 저장소(Base) 경로
target: gemini-cli           # 🤖 빌드 타겟 (gemini-cli, claude-code 등)
exclude:
  - "*.tmp"                  # 🚫 제외할 패턴 (선택 사항)

resources:
  commands:
    - my_plugin:web_search   # [플러그인]:[리소스명]
  skills:
    - shared:python_expert
```

### 3. 빌드: 베이스에서 프로젝트로 (Base → Project)

설정이 완료되었다면 한 줄만 실행하세요.

```bash
agb build
```

중앙 저장소의 검증된 리소스들이 현재 프로젝트 에이전트 규격에 맞게 즉시 생성됩니다.

### 4. 동기화: 프로젝트에서 베이스로 (Project → Base)

세션 중에 프롬프트를 튜닝하셨나요? 망설이지 말고 동기화하세요.

```bash
agb sync
```

수정된 내용이 원본 소스에 안전하게 반영되어, 다음 프로젝트에서도 그 성과를 그대로 이어갈 수 있습니다.

## 📂 프로젝트 구조

`agb`는 중앙 리소스 저장소(Source)와 실제 개발 환경(Project)을 분리하여 관리함으로써, 엄격한 관리와 유연한 확장을 동시에 보장합니다.

### 1. 중앙 리소스 저장소 (Base)

나만의 프롬프트와 스킬을 관리하는 마스터 라이브러리입니다.

```text
[Base Directory]/
├── AGENTS.md               # 공용 시스템 지침 (원본)
└── plugins/                # 플러그인 단위 리소스 모음
    └── my_plugin/
        ├── commands/       # [name].md (+ 선택적 .yaml)
        ├── agents/         # [name].md (+ 선택적 .yaml)
        └── skills/         # [name]/SKILL.md (+ 추가 파일들)
```

### 2. 프로젝트 개발 환경 (Project)

`agb build`를 통해 생성되어 에이전트가 즉시 사용하는 결과물입니다.

```text
[Project Root]/
├── agb.yaml                # 프로젝트 빌드 및 동기화 설정
├── GEMINI.md               # 변환된 전역 지침 (타겟에 따라 이름 변경)
├── commands/               # 변환된 커맨드들
├── agents/                 # 변환된 에이전트들
└── skills/                 # 변환된 스킬 폴더들
```

## 📖 더 알아보기

더 상세한 기술 규격과 설계 철학이 궁금하신가요?

- [**PRD.md**](./specs/PRD.md): 제품 요구사항 및 비즈니스 목표
- [**DESIGN.md**](./specs/DESIGN.md): 시스템 아키텍처 및 내부 설계
- [**SPEC.md**](./specs/SPEC.md): 상세 기술 규격 및 변환 규칙

