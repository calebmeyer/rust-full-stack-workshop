use shared::models::{CreateFilm, Film};
use uuid::Uuid;

pub mod postgres_film_repository;
pub use postgres_film_repository::PostgresFilmRepository;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_all(&self) -> FilmResult<Vec<Film>>;
    async fn get_by_id(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create(&self, body: &CreateFilm) -> FilmResult<Film>;
    async fn update(&self, body: &Film) -> FilmResult<Film>;
    async fn destroy(&self, id: &Uuid) -> FilmResult<Uuid>;
}
