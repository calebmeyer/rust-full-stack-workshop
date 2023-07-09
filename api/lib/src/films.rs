use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(index))
            .route("/{id}", web::get().to(show))
            .route("", web::post().to(create))
            .route("", web::put().to(update))
            .route("/{id}", web::delete().to(destroy)),
    );
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn show() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn create() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn update() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn destroy() -> HttpResponse {
    HttpResponse::Ok().finish()
}
