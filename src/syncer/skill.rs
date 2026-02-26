use crate::syncer::planner::{SyncAction, SyncPlanner};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct SkillSyncer;

impl SkillSyncer {
    /// 스킬 디렉토리 내의 추가 파일들(Extra Files)의 변경사항을 소스로 동기화합니다.
    /// SKILL.md는 Syncer의 공통 로직에서 처리하므로 여기서는 무시됩니다.
    pub fn sync_skill_dir(source_dir: &Path, target_dir: &Path, exclude_patterns: &[String]) -> Result<()> {
        let planner = SyncPlanner::new(exclude_patterns)?;
        let actions = planner.plan(source_dir, target_dir)?;

        for action in actions {
            match action {
                SyncAction::Add {
                    relative_path,
                    target_path,
                } => {
                    let source_path = source_dir.join(relative_path);
                    if let Some(parent) = source_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(target_path, &source_path)?;
                    println!("Added new file to source: {:?}", source_path);
                }
                SyncAction::Update {
                    relative_path,
                    target_path,
                } => {
                    let source_path = source_dir.join(relative_path);
                    fs::copy(target_path, &source_path)?;
                    println!("Updated file in source: {:?}", source_path);
                }
                SyncAction::Delete {
                    relative_path: _,
                    source_path,
                } => {
                    fs::remove_file(&source_path)?;
                    println!("Removed deleted file from source: {:?}", source_path);
                }
            }
        }
        Ok(())
    }
}
