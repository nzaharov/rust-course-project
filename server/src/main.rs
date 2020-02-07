mod models;

use actix_web::{
    middleware,
    web::{get, post, resource, scope, Json, Path, Query},
    App, Error, HttpResponse, HttpServer,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct PageParams {
    size: u8,
    index: u8,
}

async fn list_systems() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("list"))
}

async fn get_sys_info_page(
    pc_id: Path<String>,
    params: Query<PageParams>,
) -> Result<HttpResponse, Error> {
    println!("{} {}", params.size, params.index);
    println!("{}", pc_id.into_inner());

    Ok(HttpResponse::Ok().finish())
}

async fn post_sys_info(snapshot: Json<models::SysInfoSnapshot>) -> Result<HttpResponse, Error> {
    println!("{:?}", snapshot);
    Ok(HttpResponse::Ok().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new("%a \"%r\" %s %b %T"))
            .service(
                scope("/api").service(
                    scope("/sysinfo")
                        .service(
                            resource("")
                                .route(get().to(list_systems))
                                .route(post().to(post_sys_info)),
                        )
                        .service(resource("/{pc_id}").route(get().to(get_sys_info_page))),
                ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
