# Technical Specification: agb (Agents Builder)

## 1. 시스템 개요

`agb`는 Rust로 작성된 CLI 도구로, 여러 플러그인에 분산된 에이전트 리소스를 수집, 필터링, 변환하여 최종 결과물을 빌드합니다.

## 2. 데이터 구조 및 설정

### 2.1 agb.yaml (설정 파일)

빌드 결과물이 생성될 디렉터리에 존재해야 하며, YAML 형식으로 작성됩니다. `source` 필드는 리소스 데이터가 있는 절대 경로를 가리킵니다.

```yaml
source: /absolute/path/to/source_resources # 필수: 소스 리소스 절대 경로
target: gemini-cli # 지원: gemini-cli, claude-code, opencode
exclude:
  - "*.kor.md"
  - "*.tmp"
resources:
  commands:
    - plugin_a:foo
    - plugin_b:bar
  agents:
    - plugin_a:researcher
  skills:
    - plugin_c:python_expert
```

### 2.2 소스 디렉터리 구조 (Source)

`source` 경로가 가리키는 곳의 구조는 다음과 같습니다. (빌드 설정인 `agb.yaml`은 포함하지 않습니다.)

```text
[source_path]/
├── AGENTS.md (Root System Prompt)
└── plugins/
    └── [plugin_name]/
        ├── commands/
        │   ├── [name].md
        │   └── [name].json
        ├── agents/
        │   ├── [name].md
        │   └── [name].json
        └── skills/
            └── [skill_name]/
                ├── METADATA.json
                └── ... (기타 파일들)
```

## 3. 핵심 로직 설계

### 3.1 리소스 로딩 및 필터링

1. **Glob 필터링**: `agb.yaml`의 `exclude` 패턴을 사용하여 `source` 경로 하위의 파일을 스캔할 때 무시합니다.
2. **네임스페이스 관리**: `[plugin_name]:[resource_name]` 형식을 사용하여 소스를 식별하지만, 빌드 결과물은 네임스페이스 없이 병합됩니다.
3. **충돌 검사**: 서로 다른 플러그인에서 같은 이름의 리소스가 선택된 경우 빌드를 중단합니다.

### 3.2 타겟별 변환 (Transformation)

- **Gemini-cli**:
  - Markdown 내용을 `commands/[name].toml`의 `prompt` 필드로 삽입.
  - JSON 메타데이터의 필드(model, description 등)를 TOML 필드로 매핑.
- **Claude-code / OpenCode**:
  - Markdown과 JSON 메타데이터를 결합한 단일 마크다운 파일로 빌드 (메타데이터는 필요 시 Frontmatter로 변환).

### 3.3 빌드 실행 (Execution)

1. **Clean 단계**: `pwd`의 `commands/`, `agents/`, `skills/` 디렉터리를 재귀적으로 삭제합니다.
2. **Merge & Emit 단계**:
   - `AGENTS.md`를 각 에이전트 규격에 맞는 메인 메모리 파일(예: `GEMINI.md`)로 복사/변환합니다.
   - 변환된 리소스를 각 폴더에 작성합니다.

## 4. 구현 상세 (Rust)

- **Config Parsing**: `serde` 및 `serde_yaml` 사용.
- **File Matching**: `glob` 또는 `ignore` 크레이트를 사용하여 `.gitignore` 스타일 패턴 처리.
- **File System**: `std::fs` 및 `walkdir` 사용.
- **CLI**: `clap`을 사용하여 명령어 인터페이스 구현.

## 5. 예외 처리

- `agb.yaml` 미존재 시 에러 메시지 출력 후 종료.
- 리소스 이름 중복 시 충돌하는 플러그인 이름을 포함한 에러 메시지 출력.
- 필수 메타데이터(`JSON`) 누락 시 경고 또는 에러 처리.
