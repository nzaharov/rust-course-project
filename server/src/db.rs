use crate::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_db_pool(connection_url: &str) -> Result<DbPool, PoolError> {
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    Pool::builder().build(connection_manager)
}

pub fn get_connection(pool: &DbPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Failed to acquire connection to db")
}

pub fn list_systems(pool: &DbPool) -> Result<Vec<String>, &'static str> {
    use crate::schema::entries::dsl::*;

    let connection = get_connection(pool)?;

    let systems = entries
        .select(pc_name)
        .distinct()
        .load(&connection)
        .map_err(|_| "Failed to get system list")?;

    Ok(systems)
}

pub fn insert_new_entry(new_entry: SysInfoSnapshotDto, pool: &DbPool) -> Result<(), &'static str> {
    use crate::schema::entries::dsl::*;

    let connection = get_connection(pool)?;

    diesel::insert_into(entries)
        .values(&new_entry)
        .execute(&connection)
        .map_err(|_| "Failed to insert")?;

    Ok(())
}

pub fn fetch_log_page_by_name(
    name: &str,
    size: i64,
    index: i64,
    pool: &DbPool,
) -> Result<Option<SysLogResponse>, &'static str> {
    use crate::schema::entries::dsl::*;

    let connection = get_connection(pool)?;

    let count: i64 = entries
        .filter(pc_name.eq(name))
        .count()
        .get_result(&connection)
        .map_err(|_| "Failed to get entry count")?;
    if count == 0 {
        return Ok(None);
    }

    let page_count = count / size + 1;
    let results = entries
        .order(id)
        .filter(pc_name.eq(name))
        .limit(size)
        .offset(size * index)
        .load(&connection)
        .map_err(|_| "Failed to get logs")?;

    Ok(Some(SysLogResponse {
        entries: results,
        page_count,
    }))
}

pub fn delete_sys_log_by_name(name: &str, pool: &DbPool) -> Result<(), &'static str> {
    use crate::schema::entries::dsl::*;
    let connection = get_connection(pool)?;

    diesel::delete(entries.filter(pc_name.eq(name)))
        .execute(&connection)
        .map_err(|_| "Failed to delete")?;

    Ok(())
}
