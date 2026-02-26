use crate::core::ResourceType;
use crate::syncer::patcher::MdPatcher;
use crate::syncer::planner::{SyncAction, SyncPlanner};
use crate::transformer::Transformer;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct SkillSyncer;

impl SkillSyncer {
    pub fn sync_skill_dir(
        source_dir: &Path,
        target_dir: &Path,
        transformer: &dyn Transformer,
        exclude_patterns: &[String],
    ) -> Result<()> {
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
                    if !source_path.exists() {
                        continue;
                    }

                    let source_content = fs::read_to_string(&source_path)?;
                    let mut patcher = MdPatcher::new(&source_content);

                    // 타겟 파일 역변환을 통해 순수 본문과 메타데이터 추출 (중첩 Frontmatter 방지)
                    let detransformed = transformer.detransform(ResourceType::Skill, &target_content)?;

                    // 1. 본문 교체
                    patcher.replace_body(&detransformed.content);

                    // 2. description 업데이트 (있을 경우만)
                    if let Some(desc) = detransformed.metadata["description"].as_str() {
                        patcher.update_description(desc);
                    }

                    if !patcher.has_changed(&source_content) {
                        continue;
                    }

                    let updated = patcher.render();
                    fs::write(&source_path, updated)?;
                    println!("Patched markdown file: {:?}", source_path);
                }
            }
        }
        Ok(())
    }
}

pub fn sync_skill_dir(
    source_dir: &Path,
    target_dir: &Path,
    transformer: &dyn Transformer,
    exclude_patterns: &[String],
) -> Result<()> {
    SkillSyncer::sync_skill_dir(source_dir, target_dir, transformer, exclude_patterns)
}
