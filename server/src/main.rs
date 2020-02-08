#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;

use actix_web::{
    middleware,
    web::{self, delete, get, post, resource, scope},
    App, Error, HttpResponse, HttpServer,
};
use db::DbPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct PageParams {
    size: u8,
    index: u8,
}

async fn get_sys_list(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let systems = web::block(move || db::list_systems(&pool))
        .await
        .map_err(|e| eprintln!("{}", e))?;

    Ok(HttpResponse::Ok().json(systems))
}

async fn get_sys_info_page(
    pool: web::Data<DbPool>,
    pc_name: web::Path<String>,
    params: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let pc_name = pc_name.into_inner();
    let result = web::block(move || {
        db::fetch_log_page_by_name(&pc_name, params.size as i64, params.index as i64, &pool)
    })
    .await
    .map_err(|e| eprintln!("{}", e))?;

    match result {
        Some(sys_log) => Ok(HttpResponse::Ok().json(sys_log)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

async fn clear_sys_entries(
    pool: web::Data<DbPool>,
    pc_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let pc_name = pc_name.into_inner();
    if pc_name.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Empty sys name"));
    }
    web::block(move || db::delete_sys_log_by_name(&pc_name, &pool))
        .await
        .map_err(|e| eprintln!("{}", e))?;
    Ok(HttpResponse::NoContent().finish())
}

async fn post_sys_info(
    pool: web::Data<DbPool>,
    snapshot: web::Json<models::SysInfoSnapshotDto>,
) -> Result<HttpResponse, Error> {
    let snapshot = snapshot.into_inner();

    web::block(move || db::insert_new_entry(snapshot, &pool))
        .await
        .map_err(|e| eprintln!("{}", e))?;

    Ok(HttpResponse::Ok().body("Post successful"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connection_url = std::env::var("DATABASE_URL").expect("Database URL not found");
    let connection_pool = db::create_db_pool(&connection_url).expect("Failed to create db pool");

    HttpServer::new(move || {
        App::new()
            .data(connection_pool.clone())
            .wrap(middleware::Logger::new("%a \"%r\" %s %b %T"))
            .service(
                scope("/api").service(
                    scope("/sysinfo")
                        .service(
                            resource("")
                                .route(get().to(get_sys_list))
                                .route(post().to(post_sys_info)),
                        )
                        .service(
                            resource("/{pc_id}")
                                .route(get().to(get_sys_info_page))
                                .route(delete().to(clear_sys_entries)),
                        ),
                ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
