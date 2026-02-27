use crate::core::SKILL_MD;
use crate::loader::filter::FileFilter;
use crate::utils::fs::calculate_hash;
use anyhow::Result;
use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use log::info;
use std::fs;

#[derive(Debug, PartialEq)]
pub enum SyncAction {
    Add {
        relative_path: PathBuf,
        target_path: PathBuf,
    },
    Update {
        relative_path: PathBuf,
        target_path: PathBuf,
    },
    Delete {
        relative_path: PathBuf,
        source_path: PathBuf,
    },
}

#[derive(Default)]
pub struct ExtraSyncer {
    filter: FileFilter,
}

impl ExtraSyncer {
    pub fn new() -> Self {
        Self {
            filter: FileFilter::new(),
        }
    }

    /// 두 디렉토리 간의 파일 변경사항을 동기화합니다.
    /// 특정 파일(예: SKILL.md)은 Syncer의 공통 로직에서 처리되므로 Syncer의 제외 패턴을 통해 무시되어야 합니다.
    pub fn sync(&self, source_dir: &Path, target_dir: &Path, patterns: &[Pattern]) -> Result<()> {
        for action in self.check_actions(source_dir, target_dir, patterns)? {
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

    /// source_dir와 target_dir를 비교하여 소스를 업데이트하기 위한 액션 목록 생성
    fn check_actions(&self, source_dir: &Path, target_dir: &Path, patterns: &[Pattern]) -> Result<Vec<SyncAction>> {
        let mut actions = Vec::new();

        // 1. target_dir 스캔하여 source_dir로 동기화 (Add/Update/PatchMarkdown)
        for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
            if let Some(action) = self.check_add_or_update(source_dir, target_dir, entry.path(), patterns)? {
                actions.push(action);
            }
        }

        // 2. source_dir 스캔하여 target_dir에 없는 파일 제거 (Delete)
        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            if let Some(action) = self.check_delete(source_dir, target_dir, entry.path(), patterns)? {
                actions.push(action);
            }
        }

        Ok(actions)
    }

    fn check_add_or_update(
        &self,
        source_dir: &Path,
        target_dir: &Path,
        path: &Path,
        patterns: &[Pattern],
    ) -> Result<Option<SyncAction>> {
        if !path.is_file() {
            return Ok(None);
        }

        let relative_path = path.strip_prefix(target_dir)?;

        // SKILL.md는 Syncer에서 직접 처리하므로 여기서는 무시
        if relative_path == Path::new(SKILL_MD) {
            return Ok(None);
        }

        // exclude 패턴 체크
        if !self.filter.is_valid(target_dir, path, patterns)? {
            return Ok(None);
        }

        let source_path = source_dir.join(relative_path);

        // 파일이 존재하지 않으면 추가
        if !source_path.exists() {
            return Ok(Some(SyncAction::Add {
                relative_path: relative_path.to_path_buf(),
                target_path: path.to_path_buf(),
            }));
        }

        // 기존 파일 수정 여부 체크 (해시 비교)
        let target_hash = calculate_hash(path)?;
        let source_hash = calculate_hash(&source_path)?;

        // 내용이 다르면 업데이트
        if target_hash != source_hash {
            return Ok(Some(SyncAction::Update {
                relative_path: relative_path.to_path_buf(),
                target_path: path.to_path_buf(),
            }));
        }

        Ok(None)
    }

    fn check_delete(
        &self,
        source_dir: &Path,
        target_dir: &Path,
        path: &Path,
        patterns: &[Pattern],
    ) -> Result<Option<SyncAction>> {
        if !path.is_file() {
            return Ok(None);
        }

        let relative_path = path.strip_prefix(source_dir)?;

        // SKILL.md는 삭제 대상 아님
        if relative_path == Path::new(SKILL_MD) {
            return Ok(None);
        }

        // exclude 패턴 체크
        if !self.filter.is_valid(source_dir, path, patterns)? {
            return Ok(None);
        }

        let target_path = target_dir.join(relative_path);
        // 대상 경로에 파일이 없으면 삭제
        if !target_path.exists() {
            return Ok(Some(SyncAction::Delete {
                relative_path: relative_path.to_path_buf(),
                source_path: path.to_path_buf(),
            }));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_planner_add_update_delete() -> Result<()> {
        let source_temp = tempdir()?;
        let target_temp = tempdir()?;
        let source_dir = source_temp.path();
        let target_dir = target_temp.path();

        // Setup Source: existing.txt, deleted.txt, SKILL.md
        fs::write(source_dir.join("existing.txt"), "content")?;
        fs::write(source_dir.join("deleted.txt"), "to be deleted")?;
        fs::write(source_dir.join(SKILL_MD), "---name: test---")?;

        // Setup Target: existing.txt (modified), new.txt, SKILL.md (modified)
        fs::write(target_dir.join("existing.txt"), "modified content")?;
        fs::write(target_dir.join("new.txt"), "new file")?;
        fs::write(target_dir.join(SKILL_MD), "---name: test--- modified")?;

        let extra = ExtraSyncer::new();
        let actions = extra.check_actions(source_dir, target_dir, &[])?;

        assert!(actions.contains(&SyncAction::Update {
            relative_path: PathBuf::from("existing.txt"),
            target_path: target_dir.join("existing.txt"),
        }));
        assert!(actions.contains(&SyncAction::Add {
            relative_path: PathBuf::from("new.txt"),
            target_path: target_dir.join("new.txt"),
        }));
        assert!(actions.contains(&SyncAction::Delete {
            relative_path: PathBuf::from("deleted.txt"),
            source_path: source_dir.join("deleted.txt"),
        }));

        // SKILL.md should be ignored by the planner
        let has_skill_md = actions.iter().any(|a| match a {
            SyncAction::Add { relative_path, .. } => relative_path == Path::new(SKILL_MD),
            SyncAction::Update { relative_path, .. } => relative_path == Path::new(SKILL_MD),
            SyncAction::Delete { relative_path, .. } => relative_path == Path::new(SKILL_MD),
        });
        assert!(!has_skill_md, "SKILL.md should be ignored by the planner");

        Ok(())
    }
}
