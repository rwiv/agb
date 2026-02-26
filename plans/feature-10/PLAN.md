# PLAN: `agb sync` (Source-Target Synchronization) Implementation Plan

## 1. 개요 (Objective)
`agb sync` 명령어를 통해 타겟 빌드 결과물(`dest`)의 변경사항을 원본 소스(`source`)에 안전하게 동기화하는 기능을 단계별로 구현합니다.

## 2. 페이즈 및 작업 목록 (Phases & Tasks)

### Phase 1: 기반 인프라 업데이트 (Infrastructure Updates)
- **Task 1-1**: `core::model::ResourceData`에 `source_path` 필드 추가 (Command/Agent는 파일, Skill은 디렉터리 경로).
- **Task 1-2**: `loader::parser::ResourceParser`에서 리소스 생성 시 원본 파일/디렉터리의 절대 경로를 저장하도록 수정.
- **Task 1-3**: `core::model::ExtraFile` 및 리소스 데이터 로딩 시 해시 계산 기능 검증. (필요 시 `sha2` 디펜던시 추가)

### Phase 2: Transformer 역변환 인터페이스 확장 (Detransform Interface)
- **Task 2-1**: `transformer::Transformer` 트레이트에 `detransform` 메서드 추가.
- **Task 2-2**: `GeminiTransformer`의 `detransform` 구현 (`commands/*.toml` -> `ResourceData`).
- **Task 2-3**: `DefaultTransformer`의 `detransform` 구현 (`*.md` -> `ResourceData`).

### Phase 3: 동기화 엔진 구현 (Diff & Apply)
- **Task 3-1**: 마크다운 파일의 `description` 필드 "부분적 업데이트(Partial Update)" 함수 구현.
- **Task 3-2**: 마크다운 본문(Content) 변경 감지 및 소스 파일 덮어쓰기 로직 구현.
- **Task 3-3**: 스킬(Skill) 디렉터리 전체 동기화 로직 구현 (추가/삭제/수정 감지 및 `exclude` 패턴 적용).
- **Task 3-4**: `syncer::Syncer` 오케스트레이터 구현 및 로그 출력 시스템 정비.

### Phase 4: CLI 통합 및 명령 실행 (CLI Integration)
- **Task 4-1**: `main.rs`에 `sync` 서브커맨드 추가.
- **Task 4-2**: `Syncer`를 실행하기 위한 빌드 컨텍스트 로딩 및 실행 제어 로직 구현.

### Phase 5: 검증 및 테스트 (Validation & Testing)
- **Task 5-1**: 개별 컴포넌트 유닛 테스트 작성 (Partial Update, De-transformer).
- **Task 5-2**: `agb sync` 전체 프로세스 E2E 테스트 작성 (fixtures 활용).
- **Task 5-3**: `exclude` 패턴 및 예외 상황(파일 손상 등) 처리 테스트.

## 3. 의존성 및 제약 (Dependencies & Constraints)
- **의존성**: `Phase 1`이 완료되어야 `Phase 2`와 `Phase 3`의 경로 기반 작업이 가능합니다.
- **제약**: `dest` 디렉터리가 존재하지 않거나 읽기 권한이 없는 경우 동기화를 수행할 수 없습니다.

## 4. 성공 기준 (Success Criteria)
- `agb sync` 실행 후, 타겟에서 수정된 `description`이 소스 `.md`의 Frontmatter에 포맷 손상 없이 반영됨.
- 타겟에서 수정된 본문 내용이 소스 파일에 정확히 덮어씌워짐.
- 스킬 폴더 내에서 새로 추가된 파일이 소스로 전파되고, 삭제된 파일이 소스에서 제거됨.
- `exclude` 패턴에 해당하는 파일은 소스로 전파되지 않고 무시됨이 로그로 확인됨.
- `AGENTS.md` 등의 메인 지침 파일은 절대 수정되지 않음.
