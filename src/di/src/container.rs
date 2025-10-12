use infrastructure::db::{refresh::postgresql::PgRefreshRepository, event::postgresql::PgEventRepository, favorite::postgresql::PgFavoriteRepository, provider::PgProvider, user::postgresql::PgUserRepository};
use use_case::services::{refresh::RefreshService, event::EventService, favorite::FavoriteService, user::UserService};

pub struct DiContainer {
        db_provider: PgProvider
}

impl DiContainer {
        pub async fn new() -> Self {
                Self {
                        db_provider: PgProvider::new().await.expect("db should construct")
                }
        }

        pub fn create_user_service(&self) -> UserService<PgUserRepository> {
                UserService::new(self.db_provider.provide_user_repository())
        }

        pub fn create_event_service(&self) -> EventService<PgEventRepository> {
                EventService::new(self.db_provider.provide_event_repository())
        }

        pub fn create_favorite_service(&self) -> FavoriteService<PgFavoriteRepository> {
                FavoriteService::new(self.db_provider.provide_favorite_repository())
        }

        pub fn create_refresh_service(&self) -> RefreshService<PgRefreshRepository> {
                RefreshService::new(self.db_provider.provide_refresh_repository())
        }
}
