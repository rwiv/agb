# Technical Specification

## 1. 시스템 개요 (System Overview)

`agb`는 Rust로 작성된 CLI 도구로, 여러 플러그인에 분산된 에이전트 리소스(Commands, Agents, Skills)를 수집, 검증, 변환하여 각 타겟 에이전트(Gemini-cli, Claude-code 등)의 규격에 맞는 최종 결과물을 빌드합니다.

## 2. 기술 스택 (Technical Stack)

- **언어:** Rust (Edition 2024)
- **CLI 프레임워크:** `clap` (v4.5, derive 기능 사용)
- **직렬화/역직렬화:**
  - `serde`: 데이터 모델링 및 `#[serde(tag = "type")]`을 통한 리소스 구분
  - `serde_yaml`: `agb.yaml` 및 메타데이터(`*.yaml`, `*.yml`) 파싱
  - `serde_json`: 메타데이터(`*.json`) 파싱
  - `toml`: Gemini-cli용 결과물 생성
- **파일 시스템 및 유틸리티:**
  - `walkdir`: 플러그인 디렉터리 재귀 탐색
  - `glob`: `agb.yaml`의 exclude 패턴 매칭
  - `shellexpand`: 경로 내 물결표(`~`) 확장 지원
  - `anyhow`: 애플리케이션 레벨 에러 처리
  - `thiserror`: 커스텀 에러 정의

## 3. 시스템 아키텍처 (Architecture)

`agb`는 **파이프라인 아키텍처**를 따르며, `builder` 모듈이 전체 공정을 오케스트레이션합니다.

### 3.1 데이터 흐름 (Data Flow)

1. **Load Config**: `agb.yaml`을 읽어 빌드 컨텍스트를 생성합니다. (`builder/config.rs`)
2. **Scan & Load**: 소스 경로의 플러그인을 스캔하고 `Resource` 객체로 로드합니다. 메타데이터는 JSON과 YAML 형식을 모두 지원합니다. (`resource/loader/`)
3. **Validate & Register**: 리소스 이름 충돌 및 중복된 메타데이터 포맷을 검증하고 레지스트리에 등록합니다. (`resource/registry.rs`)
4. **Transform**: 선택된 타겟에 맞는 `Transformer`가 리소스를 변환합니다. (`transformer/`)
5. **Emit**: 기존 결과물을 정리(Clean)하고 변환된 파일을 물리적 경로에 작성합니다. (`resource/emitter.rs`)

### 3.2 모듈 구조 (Module Structure)

| 모듈 경로 | 설명 | 문서 (README.md) |
| :--- | :--- | :--- |
| `src/main.rs` | CLI 엔트리포인트 및 실행 제어 | - |
| `src/builder/` | 빌드 파이프라인 제어 및 `agb.yaml` 관리 | [`src/builder/README.md`](../../src/builder/README.md) |
| `src/resource/` | 리소스 모델, 로딩(Loader), 등록(Registry), 출력(Emitter) | [`src/resource/README.md`](../../src/resource/README.md) |
| `src/transformer/` | 타겟별 포맷 변환 로직 (Gemini, Claude, OpenCode) | [`src/transformer/README.md`](../../src/transformer/README.md) |
| `src/utils/` | 파일 시스템 조작 등 공통 유틸리티 | - |

## 4. 데이터 모델 및 상세 설계

### 4.1 리소스 모델 (`Resource`)
리소스는 `ResourceData` 구조체를 포함하며, `Enum`을 통해 타입을 구분합니다.
- **타입**: `Command`, `Agent`, `Skill`
- **구성**: `name`, `plugin`, `content` (Markdown), `metadata` (`serde_json::Value`)

### 4.2 리소스 로딩 규칙
- **Commands & Agents**: 파일 쌍(`[name].md` + `[name].{json|yaml|yml}`)으로 구성.
- **Skills**: 폴더 기반(`skills/[skill_name]/`). 폴더명과 동일한 이름의 메타데이터 파일이 필수이며, 폴더 내의 `.md` 파일들이 본문으로 합쳐집니다.
- **메타데이터 우선순위**: 동일 리소스에 대해 JSON과 YAML이 공존할 경우 에러를 발생시킵니다.

### 4.3 보안 및 제약 사항
- **타겟 전용 파일 금지**: 플러그인 내부에는 `GEMINI.md`, `CLAUDE.md`, `OPENCODE.md`와 같은 파일이 존재할 수 없습니다. 발견 시 빌드가 중단됩니다.
- **충돌 검사**: 서로 다른 플러그인에서 동일한 이름의 리소스가 선택되어 결과물 디렉터리에서 충돌하는 경우 빌드를 실패 처리합니다.

## 5. 타겟별 변환 사양 (Transformation)

- **Gemini-cli**: 
  - `commands/[name].toml`, `agents/[name].toml`, `skills/[name].toml` 생성.
  - 마크다운 본문은 `prompt` 필드로, 메타데이터는 최상위 키로 매핑.
  - `AGENTS.md` -> `GEMINI.md` 변환.
- **Claude-code / OpenCode**: 
  - `commands/[name].md` 등 생성.
  - 메타데이터를 YAML Frontmatter로, 본문을 마크다운 내용으로 결합.
  - `AGENTS.md` -> `CLAUDE.md` 또는 `OPENCODE.md` 변환.

## 6. 예외 처리 전략

- `agb.yaml` 미존재: 명확한 가이드 메시지와 함께 종료.
- 경로 확장 실패: 시스템 홈 디렉터리 접근 불가 시 에러 처리.
- Clean 단계 실패: 권한 부족 등으로 기존 파일 삭제 불가 시 안전을 위해 빌드 중단.
