use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "data")]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
}
