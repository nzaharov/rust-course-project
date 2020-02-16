use reqwest::blocking;
use serde::{Deserialize, Serialize};

const URL: &'static str = "http://localhost:8080/api/sysinfo";

#[derive(Serialize)]
pub struct SysInfoSnapshot {
    pub pc_name: String,
    pub cpu_usage: String,
    pub mem_usage: String,
    pub recorded_at: i64,
}

#[derive(Deserialize)]
pub struct SysInfoSnapshotResponse {
    pub pc_name: String,
    pub cpu_usage: String,
    pub mem_usage: String,
    pub recorded_at: i64,
}

pub struct HttpClient {
    client: blocking::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = blocking::Client::new();
        Self { client }
    }

    pub fn get_sys_list(&self) -> Result<Vec<String>, &'static str> {
        let list = self
            .client
            .get(URL)
            .send()
            .map_err(|_| "Get failed")?
            .json::<Vec<String>>()
            .map_err(|_| "Parse failed")?;
        Ok(list)
    }

    pub fn post_sys_snapshot(
        &self,
        pc_name: &str,
        cpu_usage: &str,
        mem_usage: &str,
        timestamp: i64,
    ) -> Result<(), &'static str> {
        let snapshot = SysInfoSnapshot {
            pc_name: pc_name.to_string(),
            cpu_usage: cpu_usage.to_string(),
            mem_usage: mem_usage.to_string(),
            recorded_at: timestamp,
        };

        self.client
            .post(URL)
            .json(&snapshot)
            .send()
            .map_err(|_| "Post failed")?;

        Ok(())
    }

    pub fn get_log_page(
        &self,
        pc_name: &str,
        page_index: u8,
        page_size: u8,
    ) -> Result<Vec<SysInfoSnapshotResponse>, &'static str> {
        let res = self
            .client
            .get(&format!("{}/{}", URL, pc_name))
            .query(&[("index", page_index), ("size", page_size)])
            .send()
            .map_err(|_| "Get failed")?
            .json::<Vec<SysInfoSnapshotResponse>>()
            .map_err(|_| "Parse failed")?;

        Ok(res)
    }

    pub fn delete_logs_by_name(&self, pc_name: &str) -> Result<(), &'static str> {
        self.client
            .delete(&format!("{}/{}", URL, pc_name))
            .send()
            .map_err(|_| "Delete failed")?;

        Ok(())
    }
}
