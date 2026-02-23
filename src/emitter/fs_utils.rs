use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// 특정 파일이 써질 디렉터리가 없는 경우 상위 경로를 포함해 생성합니다.
pub fn ensure_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.exists()) {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }
    Ok(())
}
