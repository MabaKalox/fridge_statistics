use serde::{Deserialize, Serialize};
type Query = sqlx::Query<'static, sqlx::Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, sqlx::Sqlite, T>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Temp {
    pub id: i64,
    pub sensor_id: i64,
    pub temp: f32,
    created: String,
}

impl crate::utils::AsRoute for Temp {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/temps/{}", self.id).into()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialTemp {
    pub sensor_id: Option<i64>,
    pub temp: Option<f32>,
}

impl PartialTemp {
    pub fn create(&self) -> Query {
        sqlx::query(
            "INSERT INTO temps (sensor_id, temp, created) 
            VALUES ($1, $2, DATETIME('now'))",
        )
        .bind(&self.sensor_id)
        .bind(&self.temp)
    }
}

impl Temp {
    pub fn last_id() -> QueryAs<(i64,)> {
        sqlx::query_as("SELECT last_insert_rowid()")
    }
    
    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM temps WHERE id = ?").bind(id)
    }
}
