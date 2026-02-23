use std::path::PathBuf;
use anyhow::Result;
use crate::core::resource::Resource;
use crate::transformers::base::{Transformer, TransformedFile};

pub struct ClaudeTransformer;

impl Transformer for ClaudeTransformer {
    fn transform(&self, _resource: &Resource) -> Result<TransformedFile> {
        // Phase 5에서 구현 예정
        unimplemented!("Claude transformer logic is not implemented yet")
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        Ok(TransformedFile {
            path: PathBuf::from("CLAUDE.md"),
            content: content.to_string(),
        })
    }
}
