# TASK 3-3: Full Skill Directory Synchronization

## 개요 (Description)
스킬(`Skill`) 디렉터리 전체의 변경사항(신규 파일 추가, 삭제, 수정)을 원본 디렉터리에 동기화하는 로직을 구현합니다.

## 수정 파일 (Files to Modify)
- `src/syncer/diff.rs` (기존 파일 확장)

## 상세 지침 (Actionable Instructions)
1. `syncer::diff` 모듈에 `sync_skill_dir(source_dir: &Path, target_dir: &Path, exclude_patterns: &[String]) -> Result<()>` 함수를 구현합니다.
2. `target_dir`을 스캔하여 `source_dir`에 없는 파일(신규 추가)을 `source_dir`로 복사합니다.
    - 이때 `exclude` 패턴에 매칭되는 파일은 건너뛰고 로그를 출력합니다.
3. `source_dir`을 스캔하여 `target_dir`에 없는 파일(삭제된 파일)을 `source_dir`에서 제거합니다.
    - `SKILL.md`는 예외적으로 삭제하지 않고 Task 3-1, 3-2의 규칙을 따릅니다.
    - `exclude` 패턴에 매칭되는 파일은 `source_dir`에 남아있더라도 삭제하지 않습니다.
4. 양쪽 디렉터리에 모두 존재하는 파일의 해시(Task 1-3)를 비교하여 다르면 `target_dir`의 내용을 `source_dir`로 덮어씁니다.
    - 바이너리 파일 대응이 가능해야 합니다.

## 검증 방법 (Verification)
- `syncer::diff::tests`를 작성하여, 스킬 디렉터리 동기화 로직의 정확성을 검증합니다.
- 신규 파일 추가, 삭제, 해시 기반 수정이 정상적으로 처리되는지 확인합니다.
- `exclude` 패턴 필터링이 올바르게 작동하는지 검증합니다.
