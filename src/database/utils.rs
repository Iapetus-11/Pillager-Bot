use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use crate::Context;

pub fn setup_database_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Database connection to be successful")
}

pub fn get_db_conn_from_ctx(
    ctx: &Context<'_>,
) -> PooledConnection<ConnectionManager<PgConnection>> {
    ctx.data()
        .db_pool
        .get()
        .expect("A valid database connection")
}
