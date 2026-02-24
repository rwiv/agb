# Task 1.2: `fs_utils`를 최상위 `utils` 모듈로 분리

## 1. Objective (목표)

- `emitter` 모듈에 종속되어 있던 파일 시스템 유틸리티를 최상위 `utils` 모듈로 추출합니다.
- 공통 유틸리티의 재사용성을 높이고 모듈 간의 책임(Policy vs Mechanism)을 명확히 분리합니다.

## 2. Context & Files (작업 범위)

- **삭제:**
  - `src/emitter/fs_utils.rs`
- **신규 생성:**
  - `src/utils/mod.rs`
  - `src/utils/fs.rs`
- **수정할 파일:**
  - `src/main.rs` (신규 모듈 `utils` 선언)
  - `src/emitter/core.rs` (유틸리티 참조 경로 수정)
  - `src/emitter/mod.rs` (불필요한 `fs_utils` 모듈 선언 제거)

## 3. Instructions (세부 지침)

### Step 1: `utils` 모듈 스캐폴딩
`src/utils/mod.rs` 파일을 생성하고 `pub mod fs;`를 선언합니다. `src/main.rs`에서 `mod utils;`를 추가합니다.

### Step 2: 로직 이동
`src/emitter/fs_utils.rs`의 모든 내용을 `src/utils/fs.rs`로 이동합니다. 
- 주요 함수: `ensure_dir` 등

### Step 3: `emitter` 참조 수정
`src/emitter/core.rs`에서 기존에 `fs_utils::ensure_dir`을 호출하던 부분을 `crate::utils::fs::ensure_dir`로 수정합니다.

### Step 4: 불필요한 파일 정리
기능 이동이 완료된 `src/emitter/fs_utils.rs` 파일을 삭제합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- `utils` 모듈은 가급적 타 도메인 모듈(`resource`, `transformers` 등)에 의존하지 않는 순수 유틸리티 상태를 유지해야 합니다.
- 함수 시그니처를 변경하지 않아 기존 호출부의 수정을 최소화합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `src/utils/fs.rs` 파일이 생성되었고 기존 유틸리티 로직이 포함되었는가?
2. `src/emitter/fs_utils.rs` 파일이 삭제되었는가?
3. `emitter` 모듈이 `utils::fs`를 통해 정상적으로 파일을 생성하고 정리하는가?
4. `cargo build` 및 `cargo test`가 성공하는가?
