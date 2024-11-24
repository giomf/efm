use crate::candidate::Candidate;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, io::Read};

const ENDPOINT_UPDATE: &str = "update";
const ENDPOINT_STATUS: &str = "status";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Member {
    pub hostname: String,
}

#[derive(Deserialize)]
pub struct MemberStatus {
    pub hostname: String,
    pub version: String,
}

impl Member {
    pub fn update<R>(&self, reader: R, size: u64) -> Result<()>
    where
        R: Read + Send + 'static,
    {
        let url = self.create_url(ENDPOINT_UPDATE);
        let response = Client::new()
            .post(&url)
            .header("Content-Type", "application/octet-stream")
            .body(reqwest::blocking::Body::sized(reader, size))
            .send()
            .with_context(|| format!("Failed to update firmware of {}", self.hostname))?;

        // Handle the response from the server
        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to update frimware of {} ({})",
                self.hostname,
                response.status()
            );
        }

        Ok(())
    }

    pub fn status(&self) -> Result<MemberStatus> {
        let url = self.create_url(ENDPOINT_STATUS);
        let status: MemberStatus = Client::new().get(url).send()?.json()?;
        Ok(status)
    }

    fn create_url(&self, endpoint: &str) -> String {
        format!("http://{}.local/{endpoint}", self.hostname)
    }
}

impl From<Candidate> for Member {
    fn from(candidate: Candidate) -> Self {
        Self {
            hostname: candidate.hostname,
        }
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.hostname)
    }
}
