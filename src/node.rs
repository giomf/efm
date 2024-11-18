use anyhow::{Context, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const SERVICE_NAME: &str = "_efm._tcp.local.";
const SERVICE_TXT_MAC_NAME: &str = "mac";
const SERVICE_BROWSE_DURATION_SEC: u64 = 1;

#[derive(Debug, Clone, Default)]
pub struct CandidateInfo {
    pub hostname: String,
    pub address: String,
    pub mac_address: String,
}

pub fn get_candidates() -> Result<Vec<CandidateInfo>> {
    let client = Arc::new(ServiceDaemon::new().context("Failed to mdns service daemon")?);
    let nodes: Arc<Mutex<Vec<ServiceInfo>>> = Arc::new(Mutex::new(Vec::new()));
    let nodes_clone = nodes.clone();
    let receiver = client
        .browse(SERVICE_NAME)
        .context("Failed to create mdns receiver")?;

    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    nodes_clone.lock().unwrap().push(info);
                }
                _ => (),
            }
        }
    });

    thread::sleep(Duration::from_secs(SERVICE_BROWSE_DURATION_SEC));
    let candidates = nodes
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
            mac_address: candidate
                .get_property_val_str(SERVICE_TXT_MAC_NAME)
                .expect("TXT Record not found")
                .to_string(),
        })
        .collect();
    Ok(candidates)
}
