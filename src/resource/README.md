# Resource 모듈

`resource` 모듈은 `agb` (Agents Builder) 프로젝트의 핵심 데이터 모델과 리소스 처리 로직을 담당합니다. 플러그인 시스템으로부터 리소스를 스캔하고, 메모리 내 객체로 변환하며, 이름 충돌을 방지하는 레지스트리 역할을 수행합니다.

## 주요 구성 요소

### 1. 리소스 모델 (`types.rs`)
에이전트 워크플로우를 구성하는 핵심 데이터를 정의합니다.
- **Resource Enum**: `Command`, `Agent`, `Skill` 타입을 지원하며, 각 타입은 공통된 `ResourceData`를 가집니다.
- **ResourceData**: 
  - `name`: 리소스 식별자 (파일명 또는 폴더명)
  - `plugin`: 해당 리소스가 속한 플러그인 이름
  - `content`: 마크다운 형식의 본문 내용
  - `metadata`: JSON 또는 YAML 형식의 추가 설정 (내부적으로는 `serde_json::Value`로 통합 관리)
- **TransformedFile**:
  - 변환기(Transformer)에 의해 처리된 최종 결과물 데이터를 담는 구조체입니다.
  - `path`: 파일이 저장될 상대 경로 (`PathBuf`)
  - `content`: 파일에 작성될 실제 내용 (`String`)

### 2. 로더 (`loader.rs`)
파일 시스템의 물리적 파일들을 `Resource` 객체로 변환하는 파이프라인을 제공합니다.
- **`scan_plugins`**: `agb.yaml`에 명시된 `source` 경로 하위의 플러그인 루트 디렉터리를 탐색하며, `exclude` 패턴(Glob)에 해당하는 파일을 필터링합니다.
  - **PRD 제약 사항**: 플러그인 내부에는 에이전트 전용 메인 메모리 파일(`GEMINI.md`, `CLAUDE.md`, `OPENCODE.md`)이 존재할 수 없으며, 발견 시 빌드가 중단됩니다.
- **`load_resources`**: 파일 경로 구조를 분석하여 관련 있는 파일들을 하나의 리소스로 병합합니다.
  - `commands/`, `agents/`: 동일한 이름을 가진 `.md`와 메타데이터(`.json`, `.yaml`, `.yml`) 파일을 하나의 리소스로 결합합니다.
  - `skills/`: 특정 폴더 내의 `METADATA.{json,yaml,yml}`과 마크다운 파일들을 기반으로 스킬 리소스를 생성합니다.
  - **포맷 충돌 검증**: 동일한 리소스에 대해 두 종류 이상의 메타데이터 포맷이 발견될 경우(예: `foo.json`과 `foo.yaml`이 공존), 빌드 에러를 발생시킵니다.

### 3. 레지스트리 (`registry.rs`)
로드된 모든 리소스를 중앙에서 관리하고 유효성을 검증합니다.
- **중복 방지**: 서로 다른 플러그인에서 동일한 이름의 리소스가 선택된 경우, 빌드 안정성을 위해 등록 과정에서 충돌(Conflict) 에러를 발생시킵니다.
- **리소스 접근**: 빌드 프로세스의 다음 단계(Transformation)에서 `all_resources()`를 통해 필터링된 모든 리소스 목록을 제공합니다.

### 4. 이미터 (`emitter.rs`)
변환된 리소스를 물리적 파일로 출력하고 관리합니다.
- **Clean**: 빌드 시작 전, 출력 디렉터리에서 이전 빌드 결과물(`commands/`, `agents/`, `skills/`, `GEMINI.md` 등)을 안전하게 삭제하여 환경을 초기화합니다.
- **Emit**: `TransformedFile` 목록을 받아 지정된 경로에 실제 파일을 생성합니다.

## 데이터 흐름 (Data Flow)

1. **Scan**: `loader`가 플러그인 디렉터리에서 유효한 파일 목록을 추출합니다.
2. **Load**: 파일의 내용을 읽어 `Resource` 객체 배열을 생성합니다.
3. **Register**: 생성된 객체들을 `Registry`에 등록하며 이름 충돌 여부를 검사합니다.
4. **Transform**: 등록된 리소스들은 `Transformer`에 의해 각 에이전트(Gemini, Claude 등) 규격에 맞는 `TransformedFile`로 변환됩니다.
5. **Emit**: `emitter`가 변환된 결과물을 물리적 파일로 작성합니다.
