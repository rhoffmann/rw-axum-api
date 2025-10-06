use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(db_url).await?;
        Ok(Self { db })
    }
}
