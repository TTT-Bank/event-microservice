use domain::models::{event::{EventFilter, EventId, EventModel, EventOrder, EventUpdate, NewEvent}, utils::Offset};
use infrastructure::db::event::repository::EventRepository;

use crate::{Result, ServiceError};

pub struct EventService<T: EventRepository> {
        repository: T,
}

impl <T: EventRepository> EventService<T> {
        pub fn new(repository: T) -> Self {
                Self { repository }
        }

        pub async fn get(&self, id: EventId) -> Result<EventModel> {
                let res = self.repository
                        .get(id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("event".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn create(&self, event: NewEvent) -> Result<EventModel> {
                let res = self.repository
                        .create(
                                event.organizer_id as i64,
                                &event.title,
                                &event.description,
                                event.date,
                                event.cost.try_into().unwrap(),
                                &event.address
                        )
                        .await;

                match res {
                        Ok(res) => Ok(res.into()),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn list(&self, offset: Offset, filters: &[EventFilter], order_by: &[EventOrder]) -> Result<Vec<EventModel>> {
                let res = self.repository
                        .list(offset, filters, order_by)
                        .await;

                match res {
                        Ok(res) => Ok(res.into_iter().map(Into::into).collect()),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn update(&self, id: EventId, changes: EventUpdate) -> Result<EventModel> {
                let res = self.repository
                        .update(id as i64, changes)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("event".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn delete(&self, id: EventId) -> Result<EventModel> {
                let res = self.repository
                        .delete(id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("event".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }
}