use anyhow::{Context, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::member::Member;

const SERVICE_NAME: &str = "_efm._tcp.local.";
const SERVICE_BROWSE_DURATION_SEC: u64 = 1;

#[derive(Debug, Clone, Default)]
pub struct Candidate {
    pub hostname: String,
    pub version: String,
}

impl Display for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.hostname)
    }
}

pub fn get_candidates(members: &Vec<Member>) -> Result<Vec<Candidate>> {
    let client = Arc::new(ServiceDaemon::new().context("Failed to mdns service daemon")?);
    let candidates: Arc<Mutex<Vec<ServiceInfo>>> = Arc::new(Mutex::new(Vec::new()));
    let candidates_clone = candidates.clone();
    let receiver = client
        .browse(SERVICE_NAME)
        .context("Failed to create mdns receiver")?;

    thread::spawn(move || {
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
        .filter_map(|candidate| {
            let hostname = candidate.get_hostname().to_string().replace(".local.", "");
            let version = candidate.get_property_val_str("version").unwrap();
            if !members.iter().any(|member| member.hostname == hostname) {
                return Some(Candidate {
                    hostname,
                    version: version.to_string(),
                });
            }
            None
        })
        .collect();
    Ok(candidates)
}
