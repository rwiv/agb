use crate::builder::config::BuildTarget;
use crate::transformer::base::Transformer;
use crate::transformer::providers::claude::ClaudeTransformer;
use crate::transformer::providers::gemini::GeminiTransformer;
use crate::transformer::providers::opencode::OpenCodeTransformer;

/// 타겟 에이전트에 맞는 Transformer 인스턴스를 반환합니다.
pub fn get_transformer(target: &BuildTarget) -> Box<dyn Transformer> {
    match target {
        BuildTarget::GeminiCli => Box::new(GeminiTransformer),
        BuildTarget::ClaudeCode => Box::new(ClaudeTransformer),
        BuildTarget::OpenCode => Box::new(OpenCodeTransformer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::config::BuildTarget;
    use std::path::PathBuf;

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
