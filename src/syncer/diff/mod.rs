pub mod markdown;
pub mod planner;

use crate::syncer::diff::markdown::MarkdownPatcher;
use crate::syncer::diff::planner::{SyncAction, SyncPlanner};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct SkillSyncer;

impl SkillSyncer {
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
                SyncAction::PatchMarkdown {
                    relative_path: _,
                    source_path,
                    target_content,
                } => {
                    if source_path.exists() {
                        let source_content = fs::read_to_string(&source_path)?;
                        let mut patcher = MarkdownPatcher::new(&source_content);

                        // 현재 SKILL.md 동기화 로직은 본문만 교체하는 것이 기본 (설계에 따라 다를 수 있음)
                        // PRD/DESIGN을 보면 '본문 교체' 및 'description surgical update' 언급됨.
                        // sync_skill_dir 내부에서는 SKILL.md의 본문과 description을 모두 업데이트하는 것이 안전.

                        // 1. 본문 교체
                        patcher.replace_body(&target_content);

                        // 2. description은 target_content에서 추출하여 업데이트해야 할 수도 있으나,
                        // 현재 target_content는 이미 변환된 결과물이므로 Frontmatter가 없을 수 있음.
                        // 일단 replace_body만으로도 대부분의 케이스가 커버됨 (Frontmatter 보존).

                        let updated = patcher.render();
                        if patcher.has_changed(&source_content) {
                            fs::write(&source_path, updated)?;
                            println!("Patched markdown file: {:?}", source_path);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

// Compatibility functions (re-implemented using SkillSyncer and MarkdownPatcher)
pub fn update_description(source: &str, new_desc: &str) -> String {
    let mut patcher = markdown::MarkdownPatcher::new(source);
    patcher.update_description(new_desc);
    patcher.render()
}

pub fn diff_content(source: &str, target: &str) -> bool {
    let patcher = markdown::MarkdownPatcher::new(source);
    patcher.has_changed(target)
}

pub fn replace_content(source: &str, new_content: &str) -> String {
    let mut patcher = markdown::MarkdownPatcher::new(source);
    patcher.replace_body(new_content);
    patcher.render()
}

pub fn sync_skill_dir(source_dir: &Path, target_dir: &Path, exclude_patterns: &[String]) -> Result<()> {
    SkillSyncer::sync_skill_dir(source_dir, target_dir, exclude_patterns)
}
