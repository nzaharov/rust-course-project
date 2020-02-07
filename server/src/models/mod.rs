use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SysInfoSnapshot {
    pub pc_id: String,
    pub cpu_usage: String,
    pub mem_usage: String,
    pub recorded_at: u64,
}
