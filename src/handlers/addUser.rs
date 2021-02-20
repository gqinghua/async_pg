use crate::{db::addUser};
use actix_web::{web, Error, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};
use crate::models::models::User;
use crate::{ errors::errors::MyError};

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = addUser::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
