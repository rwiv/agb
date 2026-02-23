use crate::config::BuildTarget;
use crate::transformers::base::Transformer;
use crate::transformers::gemini::GeminiTransformer;
use crate::transformers::claude::ClaudeTransformer;

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
    use std::path::PathBuf;
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
