pub mod gemini;

use std::path::PathBuf;
use anyhow::Result;
use crate::core::resource::Resource;
use crate::config::BuildTarget;
use thiserror::Error;

pub use gemini::GeminiTransformer;

pub struct ClaudeTransformer;
impl Transformer for ClaudeTransformer {
    fn transform(&self, _resource: &Resource) -> Result<TransformedFile> {
        unimplemented!()
    }
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        Ok(TransformedFile {
            path: PathBuf::from("CLAUDE.md"),
            content: content.to_string(),
        })
    }
}

#[derive(Debug, Error)]
pub enum TransformerError {
    #[error("Unsupported resource type: {0}")]
    UnsupportedResourceType(String),
    
    #[error("Missing metadata: {0}")]
    MissingMetadata(String),

    #[error("Transformation failed: {0}")]
    Other(String),
}

/// 변환된 파일의 경로와 내용을 담는 구조체
pub struct TransformedFile {
    /// 결과물이 저장될 상대 경로 (예: commands/foo.toml)
    pub path: PathBuf,
    /// 변환이 완료된 파일의 실제 내용
    pub content: String,
}

/// 에이전트별 리소스 변환 인터페이스
pub trait Transformer {
    /// 개별 리소스(Command, Agent, Skill)를 타겟 포맷으로 변환합니다.
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침(AGENTS.md)을 타겟 규격의 메인 메모리 파일로 변환합니다.
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}

/// 타겟 에이전트에 맞는 Transformer 인스턴스를 반환합니다.
pub fn get_transformer(target: &BuildTarget) -> Box<dyn Transformer> {
    match target {
        BuildTarget::GeminiCli => Box::new(GeminiTransformer),
        BuildTarget::ClaudeCode => Box::new(ClaudeTransformer),
        BuildTarget::OpenCode => {
            // TODO: Phase 5에서 구현 예정
            unimplemented!("OpenCode transformer is not implemented yet")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BuildTarget;

    #[test]
    fn test_transformer_factory_filenames() {
        let gemini = get_transformer(&BuildTarget::GeminiCli);
        let claude = get_transformer(&BuildTarget::ClaudeCode);

        let g_res = gemini.transform_root_prompt("test").unwrap();
        let c_res = claude.transform_root_prompt("test").unwrap();

        assert_eq!(g_res.path, PathBuf::from("GEMINI.md"));
        assert_eq!(c_res.path, PathBuf::from("CLAUDE.md"));
    }
}

