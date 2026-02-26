use crate::syncer::planner::{SyncAction, SyncPlanner};
use anyhow::Result;
use log::info;
use std::fs;
use std::path::Path;

pub struct DirectorySyncer;

impl DirectorySyncer {
    /// 두 디렉토리 간의 파일 변경사항을 동기화합니다.
    /// 특정 파일(예: SKILL.md)은 Syncer의 공통 로직에서 처리되므로 SyncPlanner의 제외 패턴을 통해 무시되어야 합니다.
    pub fn sync(source_dir: &Path, target_dir: &Path, exclude_patterns: &[String]) -> Result<()> {
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
                    info!("Added new file to source: {:?}", source_path);
                }
                SyncAction::Update {
                    relative_path,
                    target_path,
                } => {
                    let source_path = source_dir.join(relative_path);
                    fs::copy(target_path, &source_path)?;
                    info!("Updated file in source: {:?}", source_path);
                }
                SyncAction::Delete {
                    relative_path: _,
                    source_path,
                } => {
                    fs::remove_file(&source_path)?;
                    info!("Removed deleted file from source: {:?}", source_path);
                }
            }
        }
        Ok(())
    }
}
