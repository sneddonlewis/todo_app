#[macro_use] extern crate diesel;
extern crate dotenv;
use actix_service::Service;
use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod models;
mod schema;
mod database;
mod views;
mod to_do;
mod json_serialization;
mod jwt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors_policy = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let req_future = srv.call(req);
                async {
                    let result = req_future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors_policy)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
