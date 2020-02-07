#[macro_use]
extern crate diesel;
mod db;
mod models;

use actix_web::{
    middleware,
    web::{self, delete, get, post, resource, scope},
    App, Error, HttpResponse, HttpServer,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{ser, Deserialize};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct PageParams {
    size: u8,
    index: u8,
}

async fn get_sys_list(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Could not acquire connection");

    let systems = web::block(move || db::list_systems(&connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    Ok(HttpResponse::Ok().json(systems))
}

async fn get_sys_info_page(
    pool: web::Data<DbPool>,
    pc_id: web::Path<String>,
    params: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    println!("{} {}", params.size, params.index);
    println!("{}", pc_id.into_inner());

    Ok(HttpResponse::Ok().finish())
}

async fn clear_sys_entries(
    pool: web::Data<DbPool>,
    pc_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

async fn post_sys_info(
    pool: web::Data<DbPool>,
    snapshot: web::Json<models::SysInfoSnapshotDto>,
) -> Result<HttpResponse, Error> {
    let snapshot = snapshot.into_inner();
    let connection = pool.get().expect("Could not acquire connection");

    web::block(move || db::insert_new_entry(snapshot, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    Ok(HttpResponse::Ok().body("Post successful"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connection_url = std::env::var("DATABASE_URL").expect("Database URL not found");
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    let connection_pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Could not create db pool");

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
