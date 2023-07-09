use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use super::{FilmRepository, FilmResult};

pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}

impl PostgresFilmRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_all(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as::<_, Film>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM FILMS
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| err.to_string())
    }

    async fn get_by_id(&self, id: &Uuid) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM FILMS
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())
    }

    async fn create(&self, body: &CreateFilm) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            INSERT INTO films (title, director, year, poster)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, director, year, poster, created_at, updated_at
            "#,
        )
        .bind(&body.title)
        .bind(&body.director)
        .bind(body.year as i16)
        .bind(&body.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())
    }

    async fn update(&self, body: &Film) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            UPDATE films
            SET title = $2, director = $3, year = $4, poster = $5
            WHERE id = $1
            RETURNING id, title, director, year, poster, created_at, updated_at
            "#,
        )
        .bind(body.id)
        .bind(&body.title)
        .bind(&body.director)
        .bind(body.year as i16)
        .bind(&body.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())
    }

    async fn destroy(&self, id: &uuid::Uuid) -> FilmResult<uuid::Uuid> {
        sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM FILMS
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())
    }
}
