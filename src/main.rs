use actix_service::Service;
use actix_web::{App, HttpServer};
mod views;
mod state;
mod to_do;
mod processes;
mod json_serialization;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
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
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
