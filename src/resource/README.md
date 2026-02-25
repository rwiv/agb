# Resource 모듈

`resource` 모듈은 `agb` 프로젝트의 데이터 모델 정의와 리소스 생명주기(Loading -> Registering -> Emitting)를 관리하는 핵심 계층입니다.

## 주요 구성 요소

### 1. 데이터 모델 (`model.rs`)
에이전트 리소스의 핵심 구조를 정의하며, `serde`를 통한 직렬화/역직렬화를 지원합니다.
- **BuildTarget**: 빌드 대상 플랫폼 규격(`gemini-cli`, `claude-code`, `opencode`)을 정의합니다.
- **Resource Enum**: `Command`, `Agent`, `Skill` 타입을 지원합니다. `#[serde(tag = "type")]` 설정을 통해 YAML에서 타입별로 구분됩니다.
- **ResourceData**: 
  - `name`: 리소스 이름 (파일명 또는 폴더명에서 추출)
  - `plugin`: 리소스가 소속된 플러그인 식별자
  - `content`: 마크다운 본문 내용
  - `metadata`: YAML에서 파싱된 설정 값 (`serde_json::Value`)
- **ResourceKey / ResourcePaths**: 리소스 식별 및 파일 그룹화를 위한 공통 구조체입니다.
- **TransformedFile**: 변환기(Transformer)를 거친 후 최종적으로 파일 시스템에 써야 할 데이터를 담는 구조체입니다.

### 2. 리소스 로더 (`loader/`)
파일 시스템의 플러그인 구조로부터 리소스를 탐색하고 로드합니다.
- 상세한 동작 원리는 [loader/README.md](./loader/README.md)를 참조하십시오.
- **주요 기능**: 파일 필터링(Exclude), 경로 해석(Resolver), 메타데이터 파싱.

### 3. 리소스 레지스트리 (`registry.rs`)
로드된 리소스를 중앙 관리하며 전역적인 유효성을 검증합니다.
- **충돌 방지**: 서로 다른 플러그인에서 동일한 이름의 리소스가 선택된 경우 빌드를 중단하여 안전성을 보장합니다.
- **중앙 집중식 접근**: 빌드 프로세스 전반에서 필요한 리소스를 조회할 수 있는 인터페이스를 제공합니다.

### 4. 이미터 (`emitter.rs`)
변환된 최종 결과물을 물리적 파일로 출력합니다.
- **Clean**: 빌드 시작 전, 출력 디렉터리에서 이전 빌드의 잔재(`commands/`, `agents/`, `skills/` 및 `GEMINI.md` 등)를 삭제하여 깨끗한 환경을 보장합니다.
- **Emit**: `TransformedFile` 목록을 바탕으로 필요한 디렉터리를 자동 생성하고 파일 내용을 기록합니다.

## 리소스 구성 방식

### Commands & Agents
파일 한 쌍(`[name].md` + `[name].{yaml|yml}`) 또는 Frontmatter가 포함된 단일 `[name].md` 파일로 구성됩니다.
- 본문은 프롬프트로, 메타데이터(Frontmatter 또는 외부 파일)는 에이전트 설정(모델 이름, 설명 등)으로 사용됩니다.

### Skills
폴더(`skills/[skill_name]/`) 기반으로 구성됩니다.
- `SKILL.{yaml|yml}` 형식의 메타데이터 파일이 필수입니다.
- 폴더 내의 `.md` 파일들이 스킬의 지침으로 로드됩니다.

## 데이터 흐름 (Data Flow)

1. **Load**: `ResourceLoader`가 소스 디렉터리를 스캔하여 `Resource` 객체를 생성합니다.
2. **Register**: `Registry`에 등록하며 이름 중복 여부를 검사합니다.
3. **Transform**: (외부 `transformer` 모듈) 등록된 리소스를 타겟 규격에 맞게 변환합니다.
4. **Emit**: `Emitter`가 변환된 데이터를 파일 시스템에 저장합니다.
