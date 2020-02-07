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
    size: i64,
    index: i64,
    connection: &PgConnection,
) -> Result<Option<SysLogResponse>, DbError> {
    use crate::db::schema::entries::dsl::*;

    let count: i64 = entries
        .filter(pc_name.eq(name))
        .count()
        .get_result(connection)?;
    if count == 0 {
        return Ok(None);
    }

    let page_count = count / size + 1;
    let results = entries
        .order(id)
        .filter(pc_name.eq(name))
        .limit(size)
        .offset(size * index)
        .load(connection)?;

    Ok(Some(SysLogResponse {
        entries: results,
        page_count,
    }))
}
