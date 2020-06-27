#[derive(Debug, Clone)]
pub struct App {
    pub pool: sqlx::SqlitePool,
}

impl App {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}
