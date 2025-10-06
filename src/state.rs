use sqlx::PgPool;

use crate::repositories::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub user_repository: UserRepository,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        // create connection pool
        let db = PgPool::connect(db_url).await?;

        // run pending migrations
        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository = UserRepository::new(db.clone());

        Ok(Self {
            db,
            user_repository,
        })
    }
}
