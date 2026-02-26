# DESIGN: Syncer 및 SkillSyncer 구조 개선

## 1. 현재 문제점

- **중복 패치**: `Syncer::sync_resource`에서 `SKILL.md`를 이미 패치하고 있으나, `SkillSyncer::sync_skill_dir` 내부에서도 `SyncPlanner`를 통해 `SKILL.md`를 다시 패치함.
- **책임 분산**: `SyncPlanner`가 리소스의 본문인 `SKILL.md`의 변경사항까지 관리하려 하여, `Syncer`의 일반적인 리소스 동기화 로직과 충돌함.
- **코드 중복**: `SkillSyncer` 내부에서 `detransform` 및 `MdPatcher`를 중복 호출함.
- **API 모호성**: `SkillSyncer` 구조체와 독립 함수 `sync_skill_dir`이 중복 정의되어 있음.

## 2. 개선 방향

### 2.1 책임의 명확한 분리
- **`Syncer` (Orchestrator)**: 모든 리소스(Command, Agent, Skill)의 **핵심 마크다운 파일** 및 메타데이터 동기화를 담당.
- **`SkillSyncer` (Specialist)**: 스킬 디렉토리 내의 **추가 파일(Extra Files)**들의 동기화(추가, 수정, 삭제)만 담당.
- **`SyncPlanner` (Analyzer)**: 디렉토리를 비교하여 파일의 변화를 감지하되, 리소스 본문인 `SKILL.md`는 분석 대상에서 제외하거나 무시하도록 변경.

### 2.2 파이프라인 변경

#### AS-IS
1. `Syncer::sync_resource` 실행
2. `SKILL.md` 패치 및 저장
3. `SkillSyncer::sync_skill_dir` 호출
4. `SyncPlanner`가 `SKILL.md`를 포함한 모든 파일 분석
5. `SkillSyncer`가 `SKILL.md`를 **다시** 패치 및 저장
6. `SkillSyncer`가 추가 파일들(extras) 동기화

#### TO-BE
1. `Syncer::sync_resource` 실행
2. `SKILL.md` 패치 및 저장 (기존 공통 로직 유지)
3. `SkillSyncer::sync_extras` 호출 (이름 변경 제안)
4. `SyncPlanner`가 `SKILL.md`를 제외한 나머지 파일들만 분석
5. `SkillSyncer`는 분석된 결과(Add, Update, Delete)만 수행

## 3. 상세 설계

### 3.1 `SyncPlanner` 수정 (`src/syncer/planner.rs`)
- `SyncAction::PatchMarkdown` 열거형 제거.
- `plan` 메소드에서 `SKILL.md`를 발견하면 처리하지 않고 건너뜀.

### 3.2 `SkillSyncer` 리팩토링 (`src/syncer/skill.rs`)
- `SyncAction::PatchMarkdown` 처리 로직 제거.
- 중복된 독립 함수 `sync_skill_dir` 제거.
- 함수명을 `sync_skill_dir`에서 `sync_extras` 등으로 명확화할지 검토 (호환성을 위해 내부 메소드 명만 바꿀 수도 있음). 여기서는 `sync_skill_dir` 내부 로직을 정제하는 방향으로 진행.

### 3.3 `Syncer` 호출부 수정 (`src/syncer/mod.rs`)
- `skill::sync_skill_dir` 호출 시의 인자나 흐름이 변경될 경우 대응. (현재 구조 유지 가능)
