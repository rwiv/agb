# Core 모듈

## 개요
`core` 모듈은 `agb` 프로젝트의 의존성 구조에서 최하단에 위치하며, 시스템 전역에서 사용하는 공용 데이터 모델과 타입을 정의합니다. 다른 모든 모듈(`loader`, `builder`, `transformer`)은 이 모듈에 정의된 타입을 기초로 동작합니다.

## 주요 구성 요소

### 1. 상수 정의 (`constants.rs`)
프로젝트 전반에서 사용되는 규격 및 설정값들을 중앙 관리합니다.
- **파일 이름**: `AGENTS.md`, `SKILL.md`, `GEMINI.md` 등
- **디렉터리 구조**: `commands`, `agents`, `skills`
- **타겟 및 확장자**: `gemini-cli`, `.md`, `.toml`, `.yaml` 등

### 2. 데이터 모델 (`model.rs` 및 `target.rs`)
에이전트 리소스의 핵심 구조와 상태를 정의합니다.

- **ResourceType**: 리소스의 종류(`Command`, `Agent`, `Skill`)를 정의하는 열거형입니다. `Display` 트레이트를 구현하여 에러 메시지 등에서 가독성 있는 문자열을 제공합니다.
- **MetadataMap**: `map.yaml` 파일을 표현하는 모델로, 필드 이름과 원본 값을 기반으로 타겟별 치환 값을 정의합니다.
- **BuildTarget**: 빌드 대상 플랫폼 규격(`gemini-cli`, `claude-code`, `opencode`)을 정의하는 열거형입니다. 각 타겟별 예약어 키(`reserved_key`)와 전체 예약어 목록(`all_reserved_keys`)을 관리합니다.
- **Resource**: `Command`, `Agent`, `Skill` 타입을 지원하는 핵심 열거형입니다. `Skill` 타입은 `SkillData` 구조체를 통해 본문 외의 추가 파일 목록을 가질 수 있습니다. `r_type()` 메서드를 통해 자신의 타입을 반환할 수 있습니다.
- **ResourceData**: 
  - `name`: 리소스 식별 이름.
  - `plugin`: 리소스가 소속된 플러그인 이름.
  - `content`: 마크다운 본문 내용.
  - `metadata`: YAML/Frontmatter에서 파싱된 설정 값 (`serde_json::Value`).
  - `source_path`: 원본 소스 파일 또는 디렉터리 경로.
- **ExtraFile**: 물리적으로 대상 디렉터리에 복사되어야 하는 추가 파일의 경로 정보(`source`, `target`)를 담는 구조체입니다.
- **TransformedResource**: 변환된 파일들(`TransformedFile`의 목록)과 단순 복사될 추가 파일들(`ExtraFile`의 목록)을 묶은 최종 결과물 단위입니다.

### 3. 설정 모델 (`config.rs`)
`agb.yaml` 파일을 파싱하여 애플리케이션의 빌드 및 동기화 설정(`Config`, `Resources`)을 메모리 모델로 관리합니다.
- 파일의 존재 여부 및 YAML 규격을 검증하고, 경로 내의 물결표(`~`) 등 환경 변수를 확장하여 유효한 경로를 구성합니다.

## 용어 사전 (Glossary)

- **Source (소스)**: `plugins/` 디렉터리 내에 위치한 원본 마크다운 및 메타데이터 파일. 모든 빌드의 기초가 되는 단일 진실 공급원(SSOT)입니다.
- **Target (타겟)**: `gemini-cli`, `claude-code` 등 빌드 결과물이 배포될 대상 에이전트 환경 또는 해당 플랫폼의 규격.
- **Emit (배포)**: 변환된 리소스를 물리적인 파일로 대상 디렉터리에 작성하거나 복사하는 빌드의 최종 단계입니다.

## 설계 원칙
- **무의존성**: `core` 모듈은 프로젝트 내의 다른 모듈을 참조하지 않습니다.
- **데이터 중심**: 비즈니스 로직보다는 데이터의 구조와 직렬화/역직렬화 규격을 정의하는 데 집중합니다.
