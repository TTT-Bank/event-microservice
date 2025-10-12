use domain::models::{favorite::{FavoriteEventModel, FavoriteFilter, FavoriteId, FavoriteModel, FavoriteOrder}, user::UserId, utils::Offset};
use infrastructure::db::favorite::repository::FavoriteRepository;

use crate::{Result, ServiceError};

pub struct FavoriteService<T: FavoriteRepository> {
        repository: T,
}

impl <T: FavoriteRepository> FavoriteService<T> {
        pub fn new(repository: T) -> Self {
                Self { repository }
        }

        pub async fn get(&self, id: FavoriteId) -> Result<FavoriteEventModel> {
                let res = self.repository
                        .get(id.user_id as i64, id.event_id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("favorite".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn create(&self, id: FavoriteId) -> Result<FavoriteModel> {
                let res = self.repository
                        .create(id.user_id as i64, id.event_id as i64)
                        .await;

                match res {
                        Ok(res) => Ok(res.into()),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn list(&self, user_id: UserId, offset: Offset, filters: &[FavoriteFilter], order_by: &[FavoriteOrder]) -> Result<Vec<FavoriteEventModel>> {
                let res = self.repository
                        .list(user_id as i64, offset, filters, order_by)
                        .await;

                match res {
                        Ok(res) => Ok(res.into_iter().map(Into::into).collect()),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn delete(&self, id: FavoriteId) -> Result<FavoriteModel> {
                let res = self.repository
                        .delete(id.user_id as i64, id.event_id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("favorite".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }
}