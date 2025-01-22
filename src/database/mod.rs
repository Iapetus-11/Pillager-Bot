pub mod models;

pub type Db = sqlx::Pool<sqlx::Postgres>;
