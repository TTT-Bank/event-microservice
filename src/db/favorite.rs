use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, types::Json};

use super::event::{EventModel, Address, Status};
use super::utils::Offset;
use super::error::Result;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct FavoriteModel {
        pub event: EventModel,
        pub favorite_created_at: time::PrimitiveDateTime,
        pub favorite_updated_at: time::PrimitiveDateTime
}

pub async fn insert(pool: Pool<Postgres>, event_id: i64, user_id: i64) -> Result<()> {
        sqlx::query!(
                r#"
                INSERT INTO "favorite" (user_id, event_id)
                VALUES ($1, $2)
                "#,
                user_id,
                event_id
        )
        .execute(&pool)
        .await?;

        Ok(())
}

pub async fn get_all_by_user(pool: Pool<Postgres>, user_id: i64, offset: Offset) -> Result<Vec<FavoriteModel>> {
        let rows = sqlx::query!( // TODO: Make it into query_as!()
        r#"
        SELECT
                "event".id,
                "event".organizer_id,
                "event".title,
                "event".description,
                "event".date,
                "event".cost,
                "event".address AS "address: Json<Address>",
                "event".status AS "status: Status",
                "event".created_at, "event".updated_at,
                "favorite".created_at AS favorite_created_at,
                "favorite".updated_at AS favorite_updated_at
        FROM "favorite"
        JOIN "event" ON "favorite".event_id = "event".id
        WHERE "favorite".user_id = $1
        LIMIT $2
        OFFSET ($2::INT * ($3::INT - 1))
        "#,
        user_id, offset.limit, offset.page
        )
        .fetch_all(&pool)
        .await?;

        let favorites = rows.into_iter()
        .map(|r| FavoriteModel {
                event: EventModel {
                        id: r.id,
                        organizer_id: r.organizer_id,
                        title: r.title.into(),
                        description: r.description.into(),
                        date: r.date,
                        cost: r.cost,
                        address: r.address,
                        status: r.status,
                        created_at: r.created_at,
                        updated_at: r.updated_at
                },
                favorite_created_at: r.favorite_created_at,
                favorite_updated_at: r.favorite_updated_at,
        })
        .collect::<Vec<_>>();

        Ok(favorites)
}

pub async fn delete(pool: Pool<Postgres>, event_id: i64, user_id: i64) -> Result<()> {
        sqlx::query!(
                r#"
                DELETE FROM "favorite"
                WHERE user_id = $1
                AND event_id = $2
                "#,
                user_id,
                event_id
        )
        .execute(&pool)
        .await?;

        Ok(())
}