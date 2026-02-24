# Task 4.1: 빌드 디렉터리 정리 및 파일 쓰기 (Emitter)

## 1. Objective (목표)

- 변환된 리소스를 파일 시스템에 물리적으로 기록하는 `Emitter` 모듈을 구현합니다.
- 새로운 빌드를 시작하기 전 기존에 생성된 디렉터리(`commands/`, `agents/`, `skills/`)를 안전하게 정리(Clean)하고, 대상 규격에 맞는 폴더 구조를 생성합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `docs/specs/SPEC.md` (빌드 실행 및 Clean 단계 정의 확인)
  - `docs/specs/SPEC.md` (Emitter 모듈 구조 및 에러 처리 전략 확인)
- **생성 및 수정할 파일:**
  - `src/emitter/mod.rs` (신규 생성: Emitter 트레이트 및 구조체 정의)
  - `src/emitter/fs_utils.rs` (신규 생성: 파일 시스템 조작 유틸리티)

## 3. Instructions (세부 지침)

### Step 1: `Emitter` 구조체 및 핵심 메서드 구현

빌드 대상 경로를 관리하고 파일을 작성하는 로직을 구현하세요.

- **Emitter Struct:** 출력 루트 경로(`output_path`)를 상태로 가집니다.
- **`clean()` 메서드:** `output_path` 내의 `commands/`, `agents/`, `skills/`, 그리고 메인 메모리 파일(예: `GEMINI.md`)을 삭제합니다. 디렉터리가 존재하지 않는 경우 에러 없이 넘어갑니다.
- **`emit()` 메서드:** `TransformedFile` 목록을 받아 각 파일의 `path`에 맞게 실제 파일 시스템에 내용을 기록합니다.

### Step 2: 파일 시스템 유틸리티 구현 (`src/emitter/fs_utils.rs`)

안전한 파일 쓰기를 위한 도우미 함수를 작성하세요.

- **`ensure_dir()`:** 특정 파일이 써질 디렉터리가 없는 경우 `std::fs::create_dir_all`을 사용하여 상위 경로를 포함해 생성합니다.
- **에러 핸들링:** 권한 부족이나 디스크 용량 문제 등으로 파일 쓰기/삭제 실패 시 `anyhow::Result`를 통해 원인을 명확히 반환합니다.

### Step 3: 메인 메모리 파일 작성 로직

- 루트 시스템 프롬프트(`AGENTS.md`)가 변환된 결과물(예: `GEMINI.md`)을 루트 디렉터리에 작성하는 로직을 포함합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 반드시 `std::fs`와 `walkdir` 또는 `remove_dir_all` 등을 활용하여 표준적인 방식으로 구현하세요.
- 파일 쓰기 전 반드시 해당 파일이 속한 디렉터리가 존재하는지 확인하고 생성해야 합니다.
- 삭제 작업 시 사용자의 실수로 인한 데이터 유실을 방지하기 위해 지정된 특정 디렉터리(`commands`, `agents`, `skills`) 외의 파일은 건드리지 마세요.

## accept 5. Acceptance Criteria (검증 체크리스트)

1. `clean()` 실행 후 대상 디렉터리들이 완전히 삭제되거나 비워지는가?
2. `emit()` 실행 시 각 리소스가 지정된 하위 폴더(`commands/`, `agents/` 등)에 올바른 파일명으로 생성되는가?
3. 디렉터리가 존재하지 않는 상태에서 `emit()`을 호출해도 자동으로 폴더를 생성하고 파일을 쓰는가?
4. 변환된 마크다운 또는 TOML 내용이 깨짐 없이 파일에 기록되는가?
