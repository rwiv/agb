use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::BuildTarget;
use crate::core::resource::ExtraFile;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MetadataMap {
    /// Field mappings: { field_name: { original_value: { target: mapped_value } } }
    #[serde(flatten)]
    pub mappings: HashMap<String, HashMap<String, HashMap<BuildTarget, String>>>,
}

/// 변환된 파일의 경로와 내용을 담는 구조체
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformedFile {
    /// 결과물이 저장될 상대 경로 (예: commands/foo.toml)
    pub path: PathBuf,
    /// 변환이 완료된 파일의 실제 내용
    pub content: String,
}

/// 하나의 리소스에서 생성된 결과물들의 묶음
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformedResource {
    /// 변환된 텍스트 파일들
    pub files: Vec<TransformedFile>,
    /// 단순히 복사되어야 하는 추가 파일들
    pub extras: Vec<ExtraFile>,
}
