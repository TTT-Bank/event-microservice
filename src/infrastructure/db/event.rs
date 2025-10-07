use sqlx::{Pool, Postgres, QueryBuilder};
use crate::domain::event::model::*;
use crate::domain::event::repository::EventRepository;
use crate::domain::utils::Offset;

use super::error::Result;

pub struct PgEventRepository {
        pool: Pool<Postgres>
}

impl EventRepository<Pool<Postgres>> for PgEventRepository {
        type Error = sqlx::Error;

        fn init(pool: Pool<Postgres>) -> Self {
                Self { pool }
        }

        async fn create(&self, event: NewEvent) -> Result<EventModel> {
                sqlx::query_as(
                r#"
                        INSERT INTO "event" (organizer_id, title, description, date, cost, address)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        RETURNING id, organizer_id, title, description, date, cost, address, status, created_at, updated_at
                "#
                )
                .bind(event.organizer_id)
                .bind(event.title)
                .bind(event.description)
                .bind(event.date)
                .bind(event.cost as i32)
                .bind(event.address)
                .fetch_one(&self.pool)
                .await
        }

        async fn get(&self, id: EventId) -> Result<Option<EventModel>> {
                sqlx::query_as(
                r#"
                        SELECT id, organizer_id, title, description, date, cost, address, status, created_at, updated_at
                        FROM "event"
                        WHERE id = $1
                "#
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
        }

        async fn list(&self, offset: Offset, filters: &[EventFilter], order_by: &[EventOrder]) -> Result<Vec<EventModel>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"
                        SELECT id, organizer_id, title, description, date, cost, address, status, created_at, updated_at
                        FROM "event"
                        "#);

                if !filters.is_empty() {
                        let mut separated = query_builder.separated(" AND ");
                        separated.push_unseparated(" WHERE ");

                        for filter in filters {
                                match filter {
                                        EventFilter::OrganizerId(op) => separated.push("organizer_id ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value()),
                                        EventFilter::Cost(op) => separated.push("cost ").push_unseparated(op.operation() + " ").push_bind_unseparated(*op.value() as i32),
                                        EventFilter::Status(op) => separated.push("status ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value()),
                                        EventFilter::Title(op) => separated.push("title ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value()),
                                };
                        }
                }

                if !order_by.is_empty() {
                        let mut separated = query_builder.separated(", ");
                        separated.push_unseparated(" ORDER BY ");

                        for order in order_by {
                                match order {
                                        EventOrder::Id(op) => separated.push("id ").push_unseparated(op.to_string()),
                                        EventOrder::OrganizerId(op) => separated.push("organizer_id ").push_unseparated(op.to_string()),
                                        EventOrder::Status(op) => separated.push("status ").push_unseparated(op.to_string()),
                                        EventOrder::Cost(op) => separated.push("cost ").push_unseparated(op.to_string()),
                                        EventOrder::CreatedAt(op) => separated.push("created_at ").push_unseparated(op.to_string()),
                                        EventOrder::UpdatedAt(op) => separated.push("updated_at ").push_unseparated(op.to_string()),
                                };
                        }
                }

                query_builder.push(" LIMIT ").push_bind(offset.limit);
                query_builder.push(" OFFSET (").push_bind(offset.limit).push(" * (").push_bind(offset.page).push(" - 1))");

                query_builder.build_query_as().fetch_all(&self.pool).await
        }

        async fn update(&self, id: EventId, changes: EventUpdate) -> Result<Option<EventModel>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"UPDATE "event" SET "#);

                match changes {
                        EventUpdate::Title(title) => query_builder.push("title = ").push_bind(title).push(' '),
                        EventUpdate::Cost(cost) => query_builder.push("cost = ").push_bind(cost as i32).push(' '),
                        EventUpdate::Address(address) => query_builder.push("address = ").push_bind(address).push(' '),
                        EventUpdate::Status(status) => query_builder.push("status = ").push_bind(status).push(' '),
                        EventUpdate::Description(description) => query_builder.push("description = ").push_bind(description).push(' '),
                };

                query_builder.push("WHERE id = ").push_bind(id).push(' ');
                query_builder.push(
                        r#"RETURNING id, organizer_id, title, description, date, cost, address, status, created_at, updated_at"#
                );
                query_builder.build_query_as().fetch_optional(&self.pool).await
        }

        async fn delete(&self, id: EventId) -> Result<Option<EventModel>> {
                sqlx::query_as(
                r#"
                        DELETE FROM "event"
                        WHERE id = $1
                        RETURNING id, organizer_id, title, description, date, cost, address, status, created_at, updated_at
                "#
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
        }
}
