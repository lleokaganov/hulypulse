// use actix_web::{web, HttpResponse, Error};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;

use actix_web::{
    Error, HttpMessage, HttpRequest, HttpResponse, error,
    web::{self, Data, Json, Query},
};

// use hulyrs::services::jwt::Claims;

use serde::{Deserialize, Serialize};
// use tracing::{error, trace};

// type BucketPath = web::Path<(String, String)>;
// type ObjectPath = web::Path<(String, String, String)>;

pub async fn get(
    //    req: HttpRequest,
    //    path: ObjectPath,
    //    pool: Data<Pool>,
    redis: web::Data<Arc<Mutex<MultiplexedConnection>>>,
) -> Result<HttpResponse, actix_web::error::Error> {
    // ) -> Result<HttpResponse, Error> {

    let mut conn = redis.lock().await;

    let key = "lleo_key";

    // Попробуем получить значение
    let value: Option<String> = redis::cmd("GET")
        .arg(key)
        // .query_async(&mut *conn)
        .query_async::<Option<String>>(&mut *conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Записываем новое значение с TTL 5 сек
    redis::cmd("SET")
        .arg(key)
        .arg("was_here")
        .arg("EX")
        .arg(5)
        // .query_async::<_, ()>(&mut *conn)
        // .query_async(&mut *conn)
        .query_async::<()>(&mut *conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Отдаём в ответе то, что было раньше (или "empty")
    let response = value.unwrap_or_else(|| "empty".to_string());

    Ok(HttpResponse::Ok().body(response))

    // Ok(HttpResponse::Ok().body("GET response"))
}
