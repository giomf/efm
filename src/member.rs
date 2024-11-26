use crate::candidate::Candidate;
use anyhow::{Context, Result};
use chrono::{offset::Utc, DateTime};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::Read;

const ENDPOINT_UPDATE: &str = "update";
const ENDPOINT_STATUS: &str = "status";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Member {
    pub hostname: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum MemberStatus {
    Online(Member),
    Offline(Member),
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Status {
    pub hostname: String,
    pub version: String,
}

impl Member {
    pub fn new(hostname: &str, version: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            timestamp: Utc::now(),
            version: version.to_string(),
        }
    }

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

    pub fn status(&self) -> Result<Status> {
        let url = self.create_url(ENDPOINT_STATUS);
        let status: Status = Client::new().get(url).send()?.json()?;
        Ok(status)
    }

    fn create_url(&self, endpoint: &str) -> String {
        format!("http://{}.local/{endpoint}", self.hostname)
    }
}

impl From<Candidate> for Member {
    fn from(candidate: Candidate) -> Self {
        Member::new(&candidate.hostname, &candidate.version)
    }
}
