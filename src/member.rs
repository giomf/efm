use crate::candidate::Candidate;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File};

const ENDPOINT_UPDATE: &str = "update";
const ENDPOINT_STATUS: &str = "status";
const PROGRESS_BAR_TEMPLATE: &str =
    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Member {
    pub hostname: String,
}

#[derive(Deserialize)]
pub struct MemberStatus {
    pub _slot: String,
    pub version: String,
}

impl Member {
    pub fn update(&self, file_path: &str) -> Result<()> {
        let file = File::open(file_path)
            .with_context(|| format!("Failed to open firmware {}", file_path))?;
        let file_size = file.metadata()?.len();

        let progress_bar = ProgressBar::new(file_size);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(PROGRESS_BAR_TEMPLATE)?
                .progress_chars("#>-"),
        );

        let reader = progress_bar.wrap_read(file);

        let url = self.create_url(ENDPOINT_UPDATE);
        let response = Client::new()
            .post(&url)
            .header("Content-Type", "application/octet-stream")
            .body(reqwest::blocking::Body::sized(reader, file_size))
            .send()
            .with_context(|| format!("Failed to update firmware of {}", self.hostname))?;

        // Handle the response from the server
        if response.status().is_success() {
            println!("Firmware update successful!");
        } else {
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
