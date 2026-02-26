use crate::core::{Resource, SKILL_MD};
use crate::syncer::diff;
use crate::transformer::Transformer;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// 개별 리소스의 동기화 실행을 담당하는 구조체입니다.
/// 상태(output_dir, exclude)를 소유하여 라이프타임 복잡성을 제거했습니다.
pub struct Syncer {
    output_dir: PathBuf,
    exclude: Vec<String>,
}

impl Syncer {
    pub fn new(output_dir: PathBuf, exclude: Vec<String>) -> Self {
        Self { output_dir, exclude }
    }

    /// 단일 리소스를 타겟에서 소스로 동기화합니다.
    pub fn sync(&self, resource: &Resource, transformer: &dyn Transformer) -> Result<()> {
        // 타겟 파일 경로 결정 (Transformation을 통해 생성된 경로와 일치해야 함)
        let transformed = transformer
            .transform(resource)
            .with_context(|| format!("Failed to determine target path for {}", resource.name()))?;

        let target_path = self.output_dir.join(&transformed.path);
        if !target_path.exists() {
            return Ok(()); // 타겟 파일이 없으면 변경사항도 없는 것으로 간주
        }

        println!("  Checking resource: {}/{}", resource.r_type(), resource.name());

        // 타겟 파일 내용 읽기
        let target_content = fs::read_to_string(&target_path)
            .with_context(|| format!("Failed to read target file: {:?}", target_path))?;

        // 역변환 (Detransform)
        let detransformed = transformer
            .detransform(resource.r_type(), &target_content)
            .with_context(|| format!("Failed to detransform target file: {:?}", target_path))?;

        // 소스 정보 가져오기
        let (source_path, current_content, current_metadata) = match resource {
            Resource::Command(d) | Resource::Agent(d) => (&d.source_path, &d.content, &d.metadata),
            Resource::Skill(s) => (&s.base.source_path, &s.base.content, &s.base.metadata),
        };

        let mut source_file_content = match resource {
            Resource::Command(_) | Resource::Agent(_) => fs::read_to_string(source_path)?,
            Resource::Skill(_) => fs::read_to_string(source_path.join(SKILL_MD))?,
        };

        let mut changed = false;

        // 1. Description 동기화
        let old_desc = current_metadata["description"].as_str().unwrap_or_default();
        let new_desc = detransformed.metadata["description"].as_str().unwrap_or_default();

        if old_desc != new_desc {
            source_file_content = diff::update_description(&source_file_content, new_desc);
            changed = true;
            println!("    - Updated description in source");
        }

        // 2. Content 동기화
        if diff::diff_content(current_content, &detransformed.content) {
            source_file_content = diff::replace_content(&source_file_content, &detransformed.content);
            changed = true;
            println!("    - Updated content in source");
        }

        // 소스 파일 쓰기
        if changed {
            let write_path = match resource {
                Resource::Command(_) | Resource::Agent(_) => source_path.clone(),
                Resource::Skill(_) => source_path.join(SKILL_MD),
            };
            fs::write(&write_path, source_file_content)?;
        }

        // 3. Skill ExtraFiles 동기화
        if let Resource::Skill(_) = resource {
            let target_skill_dir = target_path.parent().unwrap();
            diff::sync_skill_dir(source_path, target_skill_dir, &self.exclude)?;
        }

        Ok(())
    }
}
