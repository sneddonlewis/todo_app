use actix_web::HttpResponse;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let content = read_file("templates/main.html");
    HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(content)
}
