use anyhow::{Context, Result};
use glob::Pattern;
use std::path::Path;

/// 스캔 시 파일을 필터링하는 객체입니다.
#[derive(Debug)]
pub struct FileFilter {
    patterns: Vec<Pattern>,
}

impl FileFilter {
    /// 제외 패턴 목록을 받아 FileFilter 인스턴스를 생성합니다.
    pub fn new(exclude_patterns: &[String]) -> Result<Self> {
        let mut patterns = Vec::new();
        for p in exclude_patterns {
            patterns.push(Pattern::new(p).with_context(|| format!("Invalid glob pattern: {}", p))?);
        }
        Ok(Self { patterns })
    }

    /// 파일이 필터링을 통과하여 유효한지 확인합니다.
    pub fn is_valid(&self, root: &Path, path: &Path) -> Result<bool> {
        if !path.is_file() {
            return Ok(false);
        }

        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid file name: {:?}", path))?;

        // 1. 숨김 파일 체크
        if file_name.starts_with('.') {
            return Ok(false);
        }

        // 2. PRD Constraint: 플러그인 내부 금지된 파일 체크
        if matches!(file_name, "GEMINI.md" | "CLAUDE.md" | "OPENCODE.md") {
            anyhow::bail!(
                "Forbidden file '{}' found in plugin: {:?}",
                file_name,
                path
            );
        }

        // 3. 제외 패턴 체크
        let relative_path = path.strip_prefix(root).unwrap_or(path);
        for pattern in &self.patterns {
            if pattern.matches_path(relative_path) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_file_filter_is_valid() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        
        let filter = FileFilter::new(&["*.tmp".to_string(), "ignore/".to_string()])?;

        // 유효한 파일
        let valid_file = root.join("foo.md");
        fs::write(&valid_file, "content")?;
        assert!(filter.is_valid(root, &valid_file)?);

        // 제외 패턴 (*.tmp)
        let tmp_file = root.join("test.tmp");
        fs::write(&tmp_file, "content")?;
        assert!(!filter.is_valid(root, &tmp_file)?);

        // 숨김 파일
        let hidden_file = root.join(".git");
        fs::write(&hidden_file, "content")?;
        assert!(!filter.is_valid(root, &hidden_file)?);

        Ok(())
    }

    #[test]
    fn test_forbidden_files_error() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        let filter = FileFilter::new(&[])?;

        let forbidden = ["GEMINI.md", "CLAUDE.md", "OPENCODE.md"];
        for f in forbidden {
            let path = root.join(f);
            fs::write(&path, "content")?;
            let result = filter.is_valid(root, &path);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains(&format!("Forbidden file '{}'", f)));
        }

        Ok(())
    }

    #[test]
    fn test_invalid_glob_pattern() {
        let result = FileFilter::new(&["[".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid glob pattern"));
    }
}
