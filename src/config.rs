use crate::candidate::CandidateInfo;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub members: Vec<MemberInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemberInfo {
    pub hostname: String,
    pub address: String,
    pub mac_address: String,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Config::default());
        }
        let file_content = fs::read_to_string(path)?;
        let config = toml::from_str(&file_content).context("Failed to parse config")?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file_content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(path, file_content).context("Failed to write config")?;
        Ok(())
    }
}

impl From<CandidateInfo> for MemberInfo {
    fn from(candidate: CandidateInfo) -> Self {
        Self {
            hostname: candidate.hostname,
            address: candidate.address,
            mac_address: candidate.mac_address,
        }
    }
}