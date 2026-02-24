use crate::builder::config::BuildTarget;
use crate::transformer::base::Transformer;
use crate::transformer::providers::claude::ClaudeTransformer;
use crate::transformer::providers::gemini::GeminiTransformer;
use crate::transformer::providers::opencode::OpenCodeTransformer;

/// Transformer 인스턴스를 생성하는 팩토리 객체입니다.
pub struct TransformerFactory;

impl TransformerFactory {
    /// 타겟 에이전트에 맞는 Transformer 인스턴스를 생성하여 반환합니다.
    pub fn create(target: &BuildTarget) -> Box<dyn Transformer> {
        match target {
            BuildTarget::GeminiCli => Box::new(GeminiTransformer),
            BuildTarget::ClaudeCode => Box::new(ClaudeTransformer),
            BuildTarget::OpenCode => Box::new(OpenCodeTransformer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::config::BuildTarget;
    use std::path::PathBuf;

    #[test]
    fn test_transformer_factory_filenames() {
        let gemini = TransformerFactory::create(&BuildTarget::GeminiCli);
        let claude = TransformerFactory::create(&BuildTarget::ClaudeCode);

        let g_res = gemini.transform_root_prompt("test").unwrap();
        let c_res = claude.transform_root_prompt("test").unwrap();

        assert_eq!(g_res.path, PathBuf::from("GEMINI.md"));
        assert_eq!(c_res.path, PathBuf::from("CLAUDE.md"));
    }
}
