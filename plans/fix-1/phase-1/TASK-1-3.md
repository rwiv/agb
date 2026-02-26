# TASK-1-3: SkillSyncer의 마크다운 중첩 삽입 버그 수정

## 목적
Skill 리소스 동기화 시 `SKILL.md` 파일에 타겟의 Frontmatter가 본문으로 삽입되어 파일 구조가 파괴되는 문제를 해결합니다.

## 작업 내용

1. **SkillSyncer 로직 변경 (`src/syncer/skill.rs`)**
   - `SyncAction::PatchMarkdown` 처리 시 `target_content`를 그대로 사용하지 않고, 반드시 `transformer.detransform()`을 거쳐 순수 본문과 메타데이터만 추출하도록 수정.
   - 추출된 순수 본문을 `MdPatcher::replace_body`에 전달.

2. **SyncPlanner 인터페이스 조정 (`src/syncer/planner.rs`)**
   - `PatchMarkdown` 액션이 `target_content` 대신 `target_path` 정보를 담도록 하거나, 상위 `Syncer`가 이 처리를 담당하도록 구조 변경 검토.

## 검증 방법
- `Claude` 타겟으로 빌드된 `SKILL.md`(Frontmatter 포함)를 소스로 동기화했을 때, 소스 파일의 본문만 정확히 업데이트되는지 확인.
- 소스 파일의 기존 Frontmatter 속성(예: `name`)이 유지되는지 확인.
