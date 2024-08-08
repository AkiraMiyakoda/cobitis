// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tokio::sync::OnceCell;

static POOL: OnceCell<sqlx::PgPool> = OnceCell::const_new();

pub async fn pool() -> &'static sqlx::PgPool {
    POOL.get_or_init(|| async {
        let url = std::env::var("POSTGRES_URL").expect("Envvar not found");
        sqlx::PgPool::connect(&url)
            .await
            .expect("Could not connect to PostgreSQL")
    })
    .await
}
