pub mod schema;

use crate::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn list_systems(connection: &PgConnection) -> Result<Vec<String>, diesel::result::Error> {
    use crate::db::schema::entries::dsl::*;

    let systems = entries.select(pc_name).distinct().load(connection)?;

    Ok(systems)
}

pub fn insert_new_entry(
    new_entry: SysInfoSnapshotDto,
    connection: &PgConnection,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::entries::dsl::*;

    diesel::insert_into(entries)
        .values(&new_entry)
        .execute(connection)?;

    Ok(())
}
