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

### 빌드 실행
`agb.yaml` 파일이 있는 디렉터리에서 실행합니다.
```bash
# 기본 설정(agb.yaml)으로 빌드
agb build

# 특정 설정 파일 지정
agb build --config custom-agb.yaml
```

## 설정 가이드

### 빌드 설정 (`agb.yaml`)
`agb`는 프로젝트 루트의 `agb.yaml` 파일을 통해 동작을 제어합니다.

| 필드 | 설명 |
| :--- | :--- |
| `source` | 소스 리소스 저장소의 경로 |
| `target` | 빌드 대상 에이전트 (`gemini-cli`, `claude-code` 등) |
| `resources` | 빌드에 포함할 리소스 목록 (`플러그인:이름`) |

### 리소스 작성
- **메타데이터**: Markdown 상단의 YAML Frontmatter 또는 외부 `.yaml` 파일을 사용합니다.
- **구조**: `plugins/[플러그인명]/[commands|agents|skills]/` 하위에 위치시킵니다.

## 문서 가이드
프로젝트의 상세 설계 및 기술 규격은 `specs/` 디렉토리를 참조하십시오.

- [**PRD.md**](./specs/PRD.md): 제품 요구사항 및 목표
- [**SPEC.md**](./specs/SPEC.md): 상세 기술 규격 및 변환 규칙
- [**DESIGN.md**](./specs/DESIGN.md): 시스템 아키텍처 및 상세 설계

## 라이선스
MIT License
