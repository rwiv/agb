use crate::resource::resource::Resource;
use anyhow::Result;
use std::path::PathBuf;

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
