pub mod schema;

use crate::db::schema::entries;
use crate::models::SysInfoSnapshotDto;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn insert_new_entry(
    new_entry: SysInfoSnapshotDto,
    connection: &PgConnection,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(entries::table)
        .values(&new_entry)
        .execute(connection)?;

    Ok(())
}
