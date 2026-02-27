use crate::core::{DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS};
use anyhow::Result;
use std::path::PathBuf;

/// 스캔된 리소스 정보를 담는 내부 구조체
#[derive(Debug, Clone)]
pub struct ScannedResource {
    pub plugin: String,
    pub name: String,
    pub paths: ScannedPaths,
}

impl ScannedResource {
    /// 리소스의 타입 문자열을 반환합니다.
    pub fn resource_type(&self) -> &'static str {
        match self.paths {
            ScannedPaths::Command { .. } => DIR_COMMANDS,
            ScannedPaths::Agent { .. } => DIR_AGENTS,
            ScannedPaths::Skill { .. } => DIR_SKILLS,
        }
    }

    /// 리소스의 기준 경로(source_path)를 결정합니다.
    /// Command/Agent는 md 파일 경로, Skill은 디렉터리 경로입니다.
    pub fn source_path(&self) -> Result<PathBuf> {
        let (md, _, _) = self.paths.unpack();
        match self.resource_type() {
            DIR_SKILLS => md
                .as_ref()
                .and_then(|p| p.parent())
                .ok_or_else(|| anyhow::anyhow!("Failed to determine skill root for '{}'", self.name))
                .map(|p| p.to_path_buf()),
            _ => md
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Markdown file is missing for '{}'", self.name)),
        }
    }
}

/// 리소스 타입별 파일 경로 구성
#[derive(Debug, Clone)]
pub enum ScannedPaths {
    Command {
        md: Option<PathBuf>,
        metadata: Option<PathBuf>,
    },
    Agent {
        md: Option<PathBuf>,
        metadata: Option<PathBuf>,
    },
    Skill {
        md: Option<PathBuf>,
        metadata: Option<PathBuf>,
        extras: Vec<PathBuf>,
    },
}

impl ScannedPaths {
    /// 내부 경로 정보들을 튜플로 분해하여 반환합니다.
    pub fn unpack(&self) -> (Option<PathBuf>, Option<PathBuf>, Vec<PathBuf>) {
        match self {
            ScannedPaths::Command { md, metadata } => (md.clone(), metadata.clone(), vec![]),
            ScannedPaths::Agent { md, metadata } => (md.clone(), metadata.clone(), vec![]),
            ScannedPaths::Skill { md, metadata, extras } => (md.clone(), metadata.clone(), extras.clone()),
        }
    }
}
