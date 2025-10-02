use super::model::{UserModel, UserId, UserCredentials, UserUpdate, UserFilter, UserOrder};
use super::super::utils::Offset;

pub trait UserRepository {
        type Error: std::error::Error;

        async fn get(&mut self, id: UserId) -> Result<Option<UserModel>, Self::Error>;
        async fn list(&mut self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserModel>, Self::Error>;
        async fn create(&mut self, credentials: UserCredentials) -> Result<UserModel, Self::Error>;
        async fn update(&mut self, id: UserId, changes: UserUpdate) -> Result<Option<UserModel>, Self::Error>;
        async fn delete(&mut self, id: UserId) -> Result<Option<UserModel>, Self::Error>;
}