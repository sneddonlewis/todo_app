use actix_web::web::{ServiceConfig, get};

mod items;


pub fn app_views_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(items::items));
}
