use deadpool_postgres::{Client, GenericClient};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::SensorData};

pub async fn get_data(client: &Client) -> Result<Vec<SensorData>, MyError> {
    let stmt = include_str!("../sql/get_data.sql");
    let stmt = stmt.replace("$table_fields", &SensorData::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| SensorData::from_row_ref(row).unwrap())
        .collect::<Vec<SensorData>>();

    Ok(results)
}

pub async fn post_data(client: &Client, data: SensorData) -> Result<SensorData, MyError> {
    let _stmt = include_str!("../sql/post_data.sql");
    let _stmt = _stmt.replace("$table_fields", &SensorData::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&data.temperature, &data.humidity])
        .await?
        .iter()
        .map(|row| SensorData::from_row_ref(row).unwrap())
        .collect::<Vec<SensorData>>()
        .pop()
        .ok_or(MyError::NotFound)
}
