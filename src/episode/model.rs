use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub uri: String,
    pub podcast: String,
}
