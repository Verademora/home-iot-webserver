use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db, errors::MyError, models::SensorData};

pub async fn get_data(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let data = db::get_data(&client).await?;
    Ok(HttpResponse::Ok().json(data))
}

pub async fn post_data(
    data: web::Json<SensorData>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let data: SensorData = data.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_data = db::post_data(&client, data).await?;
    Ok(HttpResponse::Ok().json(new_data))
}
