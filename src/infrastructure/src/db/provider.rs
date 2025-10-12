use std::{sync::LazyLock, time::Duration};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{Result, db::{refresh::postgresql::PgRefreshRepository, event::postgresql::PgEventRepository, favorite::postgresql::PgFavoriteRepository, user::postgresql::PgUserRepository}};

static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
        dotenvy::var("DATABASE_URL")
                .expect("DATABASE_URL var should be set")
});

static MIN_CONNECTIONS: LazyLock<u32> = LazyLock::new(|| {
        dotenvy::var("MIN_CONNECTIONS")
                .expect("MIN_CONNECTIONS var should be set")
                .parse()
                .expect("MIN_CONNECTIONS should be valid u32")
});

static MAX_CONNECTIONS: LazyLock<u32> = LazyLock::new(|| {
        dotenvy::var("MAX_CONNECTIONS")
                .expect("MAX_CONNECTIONS var should be set")
                .parse()
                .expect("MAX_CONNECTIONS should be valid u32")
});

static ACQUIRE_TIMEOUT: LazyLock<Duration> = LazyLock::new(|| {
        dotenvy::var("ACQUIRE_TIMEOUT")
                .expect("ACQUIRE_TIMEOUT var should be set")
                .parse()
                .map(Duration::from_secs)
                .expect("ACQUIRE_TIMEOUT should be valid u64")
});

static IDLE_TIMEOUT: LazyLock<Option<Duration>> = LazyLock::new(|| {
        dotenvy::var("IDLE_TIMEOUT")
                .ok()
                .map(|s|
                        s.parse()
                        .map(Duration::from_secs)
                        .expect("IDLE_TIMEOUT should be valid u64")
                )
});

static MAX_LIFETIME: LazyLock<Option<Duration>> = LazyLock::new(|| {
        dotenvy::var("MAX_LIFETIME")
                .ok()
                .map(|s|
                        s.parse()
                        .map(Duration::from_secs)
                        .expect("MAX_LIFETIME should be valid u64")
                )
});

#[derive(Debug, Clone)]
pub struct PgProvider {
        pool: Pool<Postgres>
}

impl PgProvider {
        pub async fn new() -> Result<Self> {
                let pool = PgPoolOptions::new()
                        .min_connections(*MIN_CONNECTIONS)
                        .max_connections(*MAX_CONNECTIONS)
                        .acquire_timeout(*ACQUIRE_TIMEOUT)
                        .idle_timeout(*IDLE_TIMEOUT)
                        .max_lifetime(*MAX_LIFETIME)
                        .connect(&DATABASE_URL)
                        .await?;

                sqlx::migrate!().run(&pool).await?;

                Ok(Self { pool })
        }

        pub fn provide_user_repository(&self) -> PgUserRepository {
                PgUserRepository::new(self.pool.clone())
        }

        pub fn provide_event_repository(&self) -> PgEventRepository {
                PgEventRepository::new(self.pool.clone())
        }

        pub fn provide_favorite_repository(&self) -> PgFavoriteRepository {
                PgFavoriteRepository::new(self.pool.clone())
        }

        pub fn provide_refresh_repository(&self) -> PgRefreshRepository {
                PgRefreshRepository::new(self.pool.clone())
        }
}