use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::Path;

/// 특정 파일이 써질 디렉터리가 없는 경우 상위 경로를 포함해 생성합니다.
pub fn ensure_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.exists()) {
        fs::create_dir_all(parent).with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }
    Ok(())
}

/// 파일의 SHA-256 해시를 계산하여 16진수 문자열로 반환합니다.
pub fn calculate_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file =
        fs::File::open(&path).with_context(|| format!("Failed to open file for hashing: {:?}", path.as_ref()))?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).with_context(|| format!("Failed to read file for hashing: {:?}", path.as_ref()))?;
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_calculate_hash() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");

        // 1. 초기 내용으로 해시 계산
        {
            let mut file = File::create(&file_path)?;
            file.write_all(b"hello world")?;
        }
        let hash1 = calculate_hash(&file_path)?;

        // 2. 동일한 내용이면 해시가 같아야 함
        let hash2 = calculate_hash(&file_path)?;
        assert_eq!(hash1, hash2);

        // 3. 내용이 1비트만 달라도 해시가 달라야 함
        {
            let mut file = File::create(&file_path)?;
            file.write_all(b"hello World")?; // 'w' -> 'W'
        }
        let hash3 = calculate_hash(&file_path)?;
        assert_ne!(hash1, hash3);

        Ok(())
    }
}
