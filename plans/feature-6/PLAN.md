# Plan: Metadata Overwrite & Frontmatter Extraction (Feature-6)

## 1. 개요
원본 마크다운(`.md`) 파일 내부에 정의된 Frontmatter를 기본 메타데이터로 활용하고, 별도의 메타데이터 파일(`.yaml`, `.json`)에서 빌드 타겟별로 정의된 값을 통해 이를 오버라이트하는 기능을 구현합니다. 이 문서는 구현을 담당할 에이전트가 기술적 모호함 없이 작업을 수행할 수 있도록 상세 지침을 제공합니다.

## 2. 설계 (Design)

### 2.1 Frontmatter 추출 및 분리
- **입력**: `.md` 파일 내용 (String).
- **알고리즘**:
    1. 텍스트가 `---` (유니코드/공백 주의)로 시작하는지 확인.
    2. 시작 `---` 이후의 첫 번째 `---` 위치를 찾음.
    3. 두 구분자 사이의 텍스트를 `serde_yaml`을 사용하여 `serde_json::Value`로 변환.
    4. 두 번째 `---` 이후의 텍스트를 트리밍(Trimming)하여 `content`로 반환.
- **구현 유의사항**: Frontmatter가 없는 경우 에러를 내지 않고 빈 객체(`json!({})`)와 원본 내용을 반환해야 함.

### 2.2 타겟 인지형 메타데이터 병합 (Target-Aware Merge)
- **병합 주체**: `ResourceParser` (현재 빌드 중인 `BuildTarget` 정보를 알고 있어야 함).
- **입력**:
    - `Base`: `.md`에서 추출한 Frontmatter 객체 (`serde_json::Value`).
    - `External`: 외부 메타데이터 파일에서 로드한 객체 (`serde_json::Value`).
- **병합 알고리즘**:
    1. `External`이 객체(`Value::Object`)인지 확인.
    2. 현재 `BuildTarget`에 대응하는 섹션 키를 결정:
       - `GeminiCli` -> `"gemini"`
       - `ClaudeCode` -> `"claude"`
       - `OpenCode` -> `"opencode"`
    3. **단계별 오버라이트**:
       - (1) `External` 객체의 최상위 필드 중 타겟 키들이 아닌 일반 필드(예: `name`, `description`)를 `Base`에 덮어씀.
       - (2) `External` 객체 내의 **타겟 섹션 키**에 해당하는 객체가 있다면, 그 내부 필드들을 `Base`에 덮어씀.
    4. 최종 `Base` 객체에서 타겟 키들(`gemini`, `claude`, `opencode`)은 제거함 (결과물 정돈).
- **Shallow Merge**: 모든 덮어쓰기는 최상위 키 레벨에서 발생함. 즉, 객체 내부의 특정 필드만 바꾸는 것이 아니라 키가 겹치면 해당 키의 값을 통째로 바꿈.

#### 병합 시나리오 예시 (Target: gemini-cli)
- **원본 `.md` (Frontmatter)**:
  ```yaml
  name: my-agent
  model: default-model
  temperature: 0.5
  ```
- **외부 `.yaml`**:
  ```yaml
  name: overwritten-name
  gemini:
    model: gemini-3.0-pro
    temperature: 0.2
  claude:
    model: claude-3-opus
  ```
- **최종 결과 (`metadata`)**:
  ```json
  {
    "name": "overwritten-name",
    "model": "gemini-3.0-pro",
    "temperature": 0.2
  }
  ```

## 3. 구현 단계 (Phases)

### Phase 1: 유틸리티 및 기반 로직 구현
- [ ] Frontmatter 추출 및 마크다운 분리 함수 구현 (`TASK-1-1.md`)
- [ ] 타겟 섹션을 인지하여 병합하는 오버라이트 로직 구현 (`TASK-1-2.md`)

### Phase 2: 리소스 파이프라인 통합
- [ ] `ResourceParser` 및 `ResourceLoader`에 `BuildTarget` 정보 주입 (`TASK-2-1.md`)
- [ ] `parse_resource` 메서드에 추출 및 병합 로직 결합 (`TASK-2-2.md`)

### Phase 3: 검증 및 테스트
- [ ] 다양한 조합(FM만 있음, 외부파일만 있음, 둘 다 있음)에 대한 통합 테스트 (`TASK-3-1.md`)

## 4. 성공 기준
- 빌드된 결과물의 본문(`content`)에 `---` 구분자가 포함되지 않아야 함.
- 외부 메타데이터 파일의 타겟 섹션 설정이 Frontmatter의 기본 설정을 올바르게 덮어써야 함.
- 타겟이 아닌 섹션(예: Gemini 빌드 시 Claude 섹션)은 결과물 메타데이터에 포함되지 않아야 함.
