mod models;

use actix_web::{
    middleware,
    web::{get, post, resource, scope, Json, Path},
    App, HttpServer, Responder,
};

async fn list_systems() -> impl Responder {
    "list"
}

async fn get_sys_info_page(pc_id: Path<String>) -> impl Responder {
    pc_id.into_inner()
}

async fn post_sys_info(snapshot: Json<models::SysInfoSnapshot>) -> impl Responder {
    println!("{:?}", snapshot);
    "post works"
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
