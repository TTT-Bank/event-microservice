use super::model::{UserModel, UserId, UserCredentials, UserUpdate, UserFilter, UserOrder};
use super::super::utils::Offset;

pub trait UserRepository<Db> {
        type Error: std::error::Error;

        fn init(pool: Db) -> Self;

        async fn get(&self, id: UserId) -> Result<Option<UserModel>, Self::Error>;
        async fn list(&self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserModel>, Self::Error>;
        async fn create(&self, credentials: UserCredentials) -> Result<UserModel, Self::Error>;
        async fn update(&self, id: UserId, changes: UserUpdate) -> Result<Option<UserModel>, Self::Error>;
        async fn delete(&self, id: UserId) -> Result<Option<UserModel>, Self::Error>;
}