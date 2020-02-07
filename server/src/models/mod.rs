use super::db::schema::entries;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entries"]
pub struct SysInfoSnapshotDto {
    pub pc_name: String,
    pub cpu_usage: String,
    pub mem_usage: String,
    pub recorded_at: i64,
}

#[derive(Debug, Serialize, Queryable)]
pub struct SysInfoSnapshot {
    pub id: u32,
    pub pc_name: String,
    pub cpu_usage: String,
    pub mem_usage: String,
    pub recorded_at: i64,
}
