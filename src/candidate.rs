use anyhow::{Context, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const SERVICE_NAME: &str = "_efm._tcp.local.";
const SERVICE_BROWSE_DURATION_SEC: u64 = 1;

#[derive(Debug, Clone, Default)]
pub struct CandidateInfo {
    pub hostname: String,
    pub address: String,
}

impl Display for CandidateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {}", self.hostname, self.address))
    }
}

pub fn get_candidates() -> Result<Vec<CandidateInfo>> {
    let client = Arc::new(ServiceDaemon::new().context("Failed to mdns service daemon")?);
    let candidates: Arc<Mutex<Vec<ServiceInfo>>> = Arc::new(Mutex::new(Vec::new()));
    let candidates_clone = candidates.clone();
    let receiver = client
        .browse(SERVICE_NAME)
        .context("Failed to create mdns receiver")?;

    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    candidates_clone.lock().unwrap().push(info);
                }
                _ => (),
            }
        }
    });

    thread::sleep(Duration::from_secs(SERVICE_BROWSE_DURATION_SEC));
    let candidates = candidates
        .lock()
        .unwrap()
        .iter()
        .map(|candidate| CandidateInfo {
            hostname: candidate.get_hostname().to_string(),
            address: candidate
                .get_addresses_v4()
                .iter()
                .next()
                .unwrap()
                .to_string(),
        })
        .collect();
    Ok(candidates)
}
