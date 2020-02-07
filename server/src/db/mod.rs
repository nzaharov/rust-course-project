pub mod schema;

use crate::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

type DbError = diesel::result::Error;

// struct

pub fn list_systems(connection: &PgConnection) -> Result<Vec<String>, DbError> {
    use crate::db::schema::entries::dsl::*;

    let systems = entries.select(pc_name).distinct().load(connection)?;

    Ok(systems)
}

pub fn insert_new_entry(
    new_entry: SysInfoSnapshotDto,
    connection: &PgConnection,
) -> Result<(), DbError> {
    use crate::db::schema::entries::dsl::*;

    diesel::insert_into(entries)
        .values(&new_entry)
        .execute(connection)?;

    Ok(())
}

pub fn fetch_log_page_by_name(
    name: &str,
    size: u8,
    index: u8,
    connection: &PgConnection,
) -> Result<Option<SysLogResponse>, DbError> {
    use crate::db::schema::entries::dsl::*;

    // TODO
    let results = entries.filter(pc_name.eq(name)).load(connection)?;
    let count = results.len();

    if count == 0 {
        return Ok(None);
    }

    Ok(Some(SysLogResponse {
        entries: results,
        page_count: count,
    }))
}
