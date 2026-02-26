# Task 3.3: src/resource 모듈 삭제 및 문서 업데이트

## 1. Objective (목표)
- 사용하지 않는 `src/resource` 모듈을 최종 삭제하고, 프로젝트 문서에 반영된 구조와 경로 정보를 업데이트합니다.

## 2. Context & Files (작업 범위)
- **삭제할 디렉토리:** `src/resource/`
- **수정할 파일:**
  - `README.md`
  - `specs/SPEC.md`
  - `src/main.rs` (또는 `lib.rs`) (resource 모듈 선언 제거)

## 3. Instructions (세부 지침)

### Step 1: `src/resource` 디렉토리 삭제
- `src/resource/` 내의 모든 파일이 새로운 위치로 안전하게 이동되었는지 확인한 후 삭제합니다.

```bash
rm -rf src/resource/
```

### Step 2: 최상위 모듈 선언 수정
- `src/main.rs` (또는 `lib.rs`)에서 `pub mod resource;`를 제거합니다.

### Step 3: 기술 문서 업데이트
- `specs/SPEC.md`에서 모듈 구조에 대한 설명 부분을 새로운 아키텍처(`core`, `loader`, `builder`)로 변경합니다.
- `README.md`의 모듈 설명 및 디렉토리 구조도를 최신화합니다.

## 4. Constraints (제약 사항 및 금지 행동)
- `src/resource`를 삭제하기 전에 `cargo check`와 `cargo test`를 다시 한 번 수행하여 모든 참조가 제거되었는지 확인하십시오.

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/resource/` 디렉토리가 최종적으로 제거되었는가?
2. `specs/SPEC.md`의 모듈 구조 설명이 새로운 구조와 일치하는가?
3. `cargo test`가 모두 성공적으로 완료되는가?
