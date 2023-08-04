use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

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

async fn index(
    repo: actix_web::web::Data<Box<dyn crate::film_repository::FilmRepository>>,
) -> HttpResponse {
    match repo.get_all().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn show(
    repo: actix_web::web::Data<Box<dyn crate::film_repository::FilmRepository>>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match repo.get_by_id(&id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create(
    repo: actix_web::web::Data<Box<dyn crate::film_repository::FilmRepository>>,
    film_body: web::Json<CreateFilm>,
) -> HttpResponse {
    match repo.create(&film_body).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn update(
    repo: actix_web::web::Data<Box<dyn crate::film_repository::FilmRepository>>,
    film_body: web::Json<Film>,
) -> HttpResponse {
    match repo.update(&film_body).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn destroy(
    repo: actix_web::web::Data<Box<dyn crate::film_repository::FilmRepository>>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match repo.destroy(&id).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
