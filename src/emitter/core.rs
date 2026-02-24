use crate::resource::TransformedFile;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Emitter {
    output_path: PathBuf,
}

impl Emitter {
    pub fn new(output_path: impl Into<PathBuf>) -> Self {
        Self {
            output_path: output_path.into(),
        }
    }

    /// 기존에 생성된 디렉터리 및 메인 메모리 파일을 삭제합니다.
    pub fn clean(&self) -> Result<()> {
        let targets = ["commands", "agents", "skills", "GEMINI.md", "CLAUDE.md", "AGENTS.md"];

        for target in targets {
            let path = self.output_path.join(target);
            if path.exists() {
                if path.is_dir() {
                    fs::remove_dir_all(&path).with_context(|| format!("Failed to remove directory: {:?}", path))?;
                } else {
                    fs::remove_file(&path).with_context(|| format!("Failed to remove file: {:?}", path))?;
                }
            }
        }
        Ok(())
    }

    /// 변환된 파일들을 파일 시스템에 기록합니다.
    pub fn emit(&self, files: &[TransformedFile]) -> Result<()> {
        for file in files {
            let full_path = self.output_path.join(&file.path);

            // 디렉터리 생성 확인
            crate::utils::fs::ensure_dir(&full_path)?;

            // 파일 쓰기
            fs::write(&full_path, &file.content).with_context(|| format!("Failed to write file: {:?}", full_path))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_clean() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        // 더미 파일/폴더 생성
        fs::create_dir(root.join("commands"))?;
        fs::write(root.join("commands/foo.toml"), "test")?;
        fs::write(root.join("GEMINI.md"), "test")?;
        fs::write(root.join("other.txt"), "keep me")?;

        let emitter = Emitter::new(root);
        emitter.clean()?;

        assert!(!root.join("commands").exists());
        assert!(!root.join("GEMINI.md").exists());
        assert!(root.join("other.txt").exists());

        Ok(())
    }

    #[test]
    fn test_emit() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        let emitter = Emitter::new(root);
        let files = vec![
            TransformedFile {
                path: PathBuf::from("commands/foo.toml"),
                content: "content1".to_string(),
            },
            TransformedFile {
                path: PathBuf::from("GEMINI.md"),
                content: "content2".to_string(),
            },
        ];

        emitter.emit(&files)?;

        assert_eq!(fs::read_to_string(root.join("commands/foo.toml"))?, "content1");
        assert_eq!(fs::read_to_string(root.join("GEMINI.md"))?, "content2");

        Ok(())
    }
}
