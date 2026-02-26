# Task 2.2: 리소스(Markdown/JSON) 로딩 및 유효성 검사

## 1. Objective (목표)

- `Task 2.1`에서 추출된 파일 경로 리스트를 기반으로 Markdown 본문과 JSON 메타데이터를 실제로 읽어 메모리에 로드합니다.
- 읽어온 데이터를 하나의 `Resource` 객체로 병합하고, 데이터 형식의 정합성을 검증합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (리소스 소스 구조 및 메타데이터 정의 확인)
  - `specs/SPEC.md` (`Resource` 데이터 모델 및 `serde_json` 활용)
- **생성 및 수정할 파일:**
  - `src/core/types.rs` (신규 생성: Resource, Command, Agent, Skill 데이터 모델)
  - `src/core/loader.rs` (수정: 파일 로딩 및 Resource 객체 생성 로직 추가)
  - `src/core/mod.rs` (수정: resource 모듈 선언)

## 3. Instructions (세부 지침)

### Step 1: 리소스 데이터 모델 정의 (`src/core/types.rs`)

`SPEC.md` 설계를 바탕으로 데이터 구조를 정의하세요.

- **Resource Enum:** `Command`, `Agent`, `Skill` 타입을 구분할 수 있어야 합니다.
- **공통 필드:** 각 리소스는 `name` (파일명 기준), `content` (Markdown 문자열), `metadata` (`serde_json::Value`)를 포함해야 합니다.
- `Serialize`, `Deserialize`, `Debug` 트레이트를 파생(derive)시키세요.

### Step 2: Markdown 및 JSON 병합 로딩 로직 구현

- 동일한 이름을 가진 `.md` 파일과 `.json` 파일을 한 쌍으로 찾아 병합하는 로직을 `loader.rs`에 추가하세요.
- **예:** `commands/foo.md`와 `commands/foo.json`이 존재하면 이름이 `foo`인 하나의 `Command` 리소스로 로드합니다.
- **Skill 특수 처리:** `skills/[skill_name]/METADATA.json` 구조를 확인하여 디렉터리 내의 파일들을 하나의 `Skill` 리소스로 묶습니다.

### Step 3: 데이터 유효성 검사

- JSON 메타데이터가 유효한 문법을 따르는지 `serde_json`으로 검증합니다.
- 대응하는 마크다운 파일이 없거나, 필수 메타데이터 필드(예: `description`)가 누락된 경우 `anyhow` 에러를 발생시키거나 경고를 남깁니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 여러 플러그인 간의 이름 중복 체크는 다음 단계(Task 2.3)에서 수행하므로 여기서는 **개별 리소스의 완전성**에만 집중합니다.
- `std::fs::read_to_string`을 사용하여 텍스트 데이터를 안전하게 읽어옵니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. 특정 리소스의 마크다운 내용과 JSON 데이터가 하나의 `Resource` 객체로 누락 없이 매핑되는가?
2. `Skill` 리소스의 특수한 폴더 구조(`METADATA.json`)가 정상적으로 인식되어 로드되는가?
3. 잘못된 형식의 JSON 파일이 발견되었을 때 빌드 과정에서 적절한 에러를 반환하는가?
4. 모든 구조체와 열거형이 적절한 가시성(`pub`)을 가지고 있는가?
