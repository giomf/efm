use crate::{candidate::Candidate, member::Member};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub members: Vec<Member>,
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

    pub fn adopt(&mut self, candidates: &Vec<Candidate>) {
        let mut members = candidates
            .into_iter()
            .map(|candidate| Member::new(&candidate.hostname, &candidate.version))
            .collect();
        self.members.append(&mut members);
    }

    pub fn forget(&mut self, members_to_remove: &Vec<Member>) {
        self.members
            .retain(|member| !members_to_remove.contains(&member))
    }

    pub fn update(&mut self, member_to_update: Member, version: String) {
        let member_to_update = self
            .members
            .iter_mut()
            .find(|member| **member == member_to_update)
            .unwrap();
        member_to_update.version = version;
    }
}
