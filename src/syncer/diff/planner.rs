use crate::core::SKILL_MD;
use crate::loader::filter::FileFilter;
use crate::utils::fs::calculate_hash;
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    PatchMarkdown {
        relative_path: PathBuf,
        source_path: PathBuf,
        target_content: String,
    },
}

pub struct SyncPlanner {
    filter: FileFilter,
}

impl SyncPlanner {
    pub fn new(exclude_patterns: &[String]) -> Result<Self> {
        Ok(Self {
            filter: FileFilter::new(exclude_patterns)?,
        })
    }

    /// source_dir와 target_dir를 비교하여 소스를 업데이트하기 위한 액션 목록 생성
    pub fn plan(&self, source_dir: &Path, target_dir: &Path) -> Result<Vec<SyncAction>> {
        let mut actions = Vec::new();

        // 1. target_dir 스캔하여 source_dir로 동기화 (Add/Update/PatchMarkdown)
        for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let relative_path = path.strip_prefix(target_dir)?;

            // SKILL.md는 PatchMarkdown 대상으로 분류 (나중에 SkillSyncer에서 Patcher 사용)
            if relative_path == Path::new(SKILL_MD) {
                let target_content = std::fs::read_to_string(path)?;
                actions.push(SyncAction::PatchMarkdown {
                    relative_path: relative_path.to_path_buf(),
                    source_path: source_dir.join(relative_path),
                    target_content,
                });
                continue;
            }

            // exclude 패턴 체크
            if !self.filter.is_valid(target_dir, path)? {
                continue;
            }

            let source_path = source_dir.join(relative_path);

            if !source_path.exists() {
                actions.push(SyncAction::Add {
                    relative_path: relative_path.to_path_buf(),
                    target_path: path.to_path_buf(),
                });
            } else {
                // 기존 파일 수정 여부 체크 (해시 비교)
                let target_hash = calculate_hash(path)?;
                let source_hash = calculate_hash(&source_path)?;

                if target_hash != source_hash {
                    actions.push(SyncAction::Update {
                        relative_path: relative_path.to_path_buf(),
                        target_path: path.to_path_buf(),
                    });
                }
            }
        }

        // 2. source_dir 스캔하여 target_dir에 없는 파일 제거 (Delete)
        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let relative_path = path.strip_prefix(source_dir)?;

            // SKILL.md는 삭제 대상 아님
            if relative_path == Path::new(SKILL_MD) {
                continue;
            }

            // exclude 패턴 체크
            if !self.filter.is_valid(source_dir, path)? {
                continue;
            }

            let target_path = target_dir.join(relative_path);
            if !target_path.exists() {
                actions.push(SyncAction::Delete {
                    relative_path: relative_path.to_path_buf(),
                    source_path: path.to_path_buf(),
                });
            }
        }

        Ok(actions)
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

        let planner = SyncPlanner::new(&[])?;
        let actions = planner.plan(source_dir, target_dir)?;

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

        // SKILL.md should be PatchMarkdown
        let has_patch = actions.iter().any(
            |a| matches!(a, SyncAction::PatchMarkdown { relative_path, .. } if relative_path == Path::new(SKILL_MD)),
        );
        assert!(has_patch);

        Ok(())
    }
}
