use actix_web::{HttpResponse, delete, get, post, web::{Data, Json, Path}};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::domain::user::model::UserId;
use crate::db::{event::EventId, favorite::{self, FavoriteEvent, FavoriteModel}, utils::Offset};

use super::error::Result;

pub fn favorite_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/users/{id}/favorites")
        .service(create_favorite)
        .service(delete_favorite)
        .service(list_favorites)
        );
}

#[derive(Debug, Serialize, ToSchema)]
struct FavoriteResponse {
        favorite: FavoriteModel
}


#[utoipa::path]
#[post("")]
async fn create_favorite(pool: Data<Pool<Postgres>>, user_id: Path<UserId>, event_id: Json<EventId>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let favorite =
                favorite::insert(transaction.as_mut(), *user_id, *event_id)
                        .await
                        .map_err(Into::into);
        transaction.commit().await?;

        match favorite {
                Ok(favorite) => {
                        Ok(HttpResponse::Created().json(FavoriteResponse {favorite}))
                }
                Err(err) => {
                        if let super::error::HandlerError::Db(db_err) = &err {
                                if db_err == &sqlx::error::ErrorKind::UniqueViolation {
                                        return Ok(HttpResponse::Conflict().finish())
                                }
                        }

                        return Err(err)
                }
        }
}

#[utoipa::path]
#[delete("/{id}")]
async fn delete_favorite(pool: Data<Pool<Postgres>>, id: Path<(UserId, EventId)>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let (user_id, event_id) = id.into_inner();
        let favorite = favorite::delete(transaction.as_mut(), user_id, event_id).await?;
        transaction.commit().await?;

        let res = match favorite {
                Some(favorite) => {
                        HttpResponse::NoContent().json(FavoriteResponse {favorite})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[derive(Debug, Deserialize, ToSchema)]
struct ListFavoritesjson {
        #[serde(default)]
        offset: Offset
}

#[derive(Debug, Serialize, ToSchema)]
struct ListFavoritesResponse {
        favorites: Vec<FavoriteEvent>
}

#[utoipa::path]
#[get("")]
async fn list_favorites(pool: Data<Pool<Postgres>>, user_id: Path<UserId>, json: Json<ListFavoritesjson>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let json = json.into_inner();
        let favorites = favorite::get_all_by_user(transaction.as_mut(), *user_id, json.offset).await?;
        transaction.commit().await?;

        Ok(HttpResponse::Ok().json(ListFavoritesResponse {favorites}))
}