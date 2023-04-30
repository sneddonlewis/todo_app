use actix_web::HttpResponse;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let mut content = read_file("templates/main.html");
    let js_content = read_file("templates/main.js");
    content = content.replace("{{JAVASCRIPT}}", &js_content);
    HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(content)
}
