# Task 3-1: `SkillSyncer` 구현 및 통합

## 1. 개요
`SyncPlanner`와 `MarkdownPatcher`를 사용하여 실제 동기화 작업을 수행하는 `SkillSyncer`를 구현하고, 기존 동기화 시스템에 통합합니다.

## 2. 작업 상세

### 2.1. 파일 생성
- `src/syncer/diff/mod.rs` 파일을 생성하여 하위 모듈(`markdown`, `planner`)을 선언하고 `SkillSyncer`를 구현합니다.

### 2.2. `SkillSyncer` 구현
```rust
pub struct SkillSyncer;

impl SkillSyncer {
    pub fn sync_skill_dir(source_dir: &Path, target_dir: &Path, exclude_patterns: &[String]) -> Result<()> {
        let planner = SyncPlanner::new(exclude_patterns)?;
        let actions = planner.plan(source_dir, target_dir)?;
        
        for action in actions {
            match action {
                SyncAction::Add { .. } => { /* fs::copy */ },
                SyncAction::Update { .. } => { /* fs::copy */ },
                SyncAction::Delete { .. } => { /* fs::remove_file */ },
                SyncAction::PatchMarkdown { .. } => { 
                    // MarkdownPatcher 사용
                },
            }
        }
        Ok(())
    }
}
```

### 2.3. 기존 호출부 업데이트
- `src/syncer/sync.rs` 등에서 기존 `diff::sync_skill_dir`을 호출하던 코드를 새로운 `SkillSyncer` 또는 `diff::sync_skill_dir`로 연결합니다.

## 3. 검증 방법
- 전체 프로젝트 빌드 및 `cargo test` 수행.
