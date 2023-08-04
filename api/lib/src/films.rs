use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(index::<R>))
            .route("/{id}", web::get().to(show::<R>))
            .route("", web::post().to(create::<R>))
            .route("", web::put().to(update::<R>))
            .route("/{id}", web::delete().to(destroy::<R>)),
    );
}

async fn index<R: FilmRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_all().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn show<R: FilmRepository>(repo: web::Data<R>, id: web::Path<Uuid>) -> HttpResponse {
    match repo.get_by_id(&id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create<R: FilmRepository>(
    repo: web::Data<R>,
    film_body: web::Json<CreateFilm>,
) -> HttpResponse {
    match repo.create(&film_body).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn update<R: FilmRepository>(repo: web::Data<R>, film_body: web::Json<Film>) -> HttpResponse {
    match repo.update(&film_body).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn destroy<R: FilmRepository>(repo: web::Data<R>, id: web::Path<Uuid>) -> HttpResponse {
    match repo.destroy(&id).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
