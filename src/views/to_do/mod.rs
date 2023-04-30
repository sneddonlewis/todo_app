mod create;
mod get;
mod edit;
mod delete;

use actix_web::web::{ServiceConfig, post, get, scope};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("", get().to(get::get_all))
            .route("create/{title}", post().to(create::create))
            .route("edit", post().to(edit::edit))
            .route("delete", post().to(delete::delete))
    );
}
