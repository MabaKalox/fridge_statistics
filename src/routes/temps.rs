use crate::records::{Temp, PartialTemp};
use crate::utils;
use sqlx::prelude::*;

pub async fn create(mut request: crate::Request) -> tide::Result {
    let db = &request.state().db;
    let mut tx = db.begin().await?;
    let temp: PartialTemp = utils::deserialize_body(&mut request).await?;
    let created = temp.create().execute(&mut tx).await?;

    if created == 1 {
        let (last_id,) = Temp::last_id().fetch_one(&mut tx).await?;
        tx.commit().await?;

        Ok(tide::Redirect::new(format!("/temps/{}", last_id)).into())
    } else {
        Ok(format!("Failed").into())
    }
}

pub async fn show(request: crate::Request) -> tide::Result {
    let temp = Temp::find_by_id(request.param("temp_id")?.parse()?)
        .fetch_one(&request.state().db)
        .await?;
    let temp_str = serde_json::to_string(&temp)?;
    
    Ok(temp_str.into())
}
