use crate::core::SKILL_MD;
use crate::loader::filter::FileFilter;
use crate::utils::fs::calculate_hash;
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// 스킬(Skill) 디렉터리 전체의 변경사항(신규 파일 추가, 삭제, 수정)을 원본 디렉터리에 동기화합니다.
pub fn sync_skill_dir(source_dir: &Path, target_dir: &Path, exclude_patterns: &[String]) -> Result<()> {
    let filter = FileFilter::new(exclude_patterns)?;

    // 1. target_dir 스캔하여 source_dir로 동기화 (추가/수정)
    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let relative_path = path.strip_prefix(target_dir)?;

        // SKILL.md는 별도 로직(description/content sync)으로 처리되므로 건너뜀
        if relative_path == Path::new(SKILL_MD) {
            continue;
        }

        // exclude 패턴 체크
        if !filter.is_valid(target_dir, path)? {
            println!("Skipping excluded file: {:?}", relative_path);
            continue;
        }

        let source_path = source_dir.join(relative_path);

        if !source_path.exists() {
            // 신규 파일 추가
            if let Some(parent) = source_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &source_path)?;
            println!("Added new file to source: {:?}", relative_path);
        } else {
            // 기존 파일 수정 여부 체크 (해시 비교)
            let target_hash = calculate_hash(path)?;
            let source_hash = calculate_hash(&source_path)?;

            if target_hash != source_hash {
                fs::copy(path, &source_path)?;
                println!("Updated file in source: {:?}", relative_path);
            }
        }
    }

    // 2. source_dir 스캔하여 target_dir에 없는 파일 제거 (삭제)
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

        // exclude 패턴 체크: 소스에 있는 파일이 제외 대상이면 삭제하지 않음
        if !filter.is_valid(source_dir, path)? {
            continue;
        }

        let target_path = target_dir.join(relative_path);
        if !target_path.exists() {
            fs::remove_file(path)?;
            println!("Removed deleted file from source: {:?}", relative_path);
        }
    }

    Ok(())
}

/// 마크다운 파일의 포맷을 손상시키지 않고 description 필드만 업데이트합니다.
pub fn update_description(source: &str, new_desc: &str) -> String {
    let content = source.trim_start();
    if !content.starts_with("---") {
        // Frontmatter가 없는 경우 새로 생성
        return format!(
            "---
description: {}
---

{}",
            new_desc, source
        );
    }

    let rest = &content[3..];
    if let Some(end_offset) = rest.find("---") {
        let yaml_part = &rest[..end_offset];
        let pure_content = &rest[end_offset + 3..];

        let mut lines: Vec<String> = yaml_part.lines().map(|s| s.to_string()).collect();
        let mut found = false;

        // description: 키를 찾아 교체
        for line in lines.iter_mut() {
            if let Some(caps) = regex::Regex::new(r"^(\s*description:\s*).*$").unwrap().captures(line) {
                let prefix = caps.get(1).unwrap().as_str();
                *line = format!("{}{}", prefix, new_desc);
                found = true;
                break;
            }
        }

        if !found {
            // 못 찾았다면 마지막에 추가
            lines.push(format!("description: {}", new_desc));
        }

        format!(
            "---
{}
---{}",
            lines.join(
                "
"
            ),
            pure_content
        )
    } else {
        // 닫는 ---가 없는 경우 (잘못된 형식), 안전하게 앞에 추가
        format!(
            "---
description: {}
---

{}",
            new_desc, source
        )
    }
}

/// 텍스트가 1글자라도 다르면 true를 반환합니다.
pub fn diff_content(source: &str, target: &str) -> bool {
    source.trim() != target.trim()
}

/// 소스 파일의 본문(Content) 부분을 타겟의 본문으로 교체합니다.
pub fn replace_content(source: &str, new_content: &str) -> String {
    let content = source.trim_start();
    if !content.starts_with("---") {
        // Frontmatter가 없는 경우 그냥 덮어씀 (단, Frontmatter가 없으므로 new_content만)
        return new_content.to_string();
    }

    let rest = &content[3..];
    if let Some(end_offset) = rest.find("---") {
        let yaml_part = &rest[..end_offset];
        // Frontmatter 영역을 유지하고 본문만 교체
        format!("---\n{}\n---\n\n{}", yaml_part, new_content)
    } else {
        // 닫는 ---가 없는 경우 그냥 덮어씀
        new_content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_content() {
        assert!(diff_content("hello", "world"));
        assert!(!diff_content("hello", "hello"));
        assert!(!diff_content("hello\n", "hello"));
    }

    #[test]
    fn test_replace_content() {
        let source = "---
name: test
---
# Old Content";
        let updated = replace_content(source, "# New Content");
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# New Content"));
        assert!(!updated.contains("# Old Content"));
    }

    #[test]
    fn test_update_description_existing() {
        let source = "---
name: test
description: old description
---
# Content";
        let updated = update_description(source, "new description");
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# Content"));
    }

    #[test]
    fn test_update_description_missing() {
        let source = "---
name: test
---
# Content";
        let updated = update_description(source, "new description");
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
    }

    #[test]
    fn test_update_description_no_frontmatter() {
        let source = "# Content";
        let updated = update_description(source, "new description");
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("# Content"));
        assert!(updated.starts_with("---"));
    }
}
