use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

use tracing::{error, trace};

use uuid::Uuid;

// type BucketPath = web::Path<(String, String)>;
// type ObjectPath = web::Path<(String, String, String)>;
type ObjectPath = web::Path<(String, String)>;

use crate::redis::{
    RedisArray,
    redis_save,
    redis_read,
    redis_delete,
};

use actix_web::{
    Error, HttpMessage, HttpRequest, HttpResponse, error,
    web::{self, Data, Json, Query},
};

/// get / (test)

pub async fn get(
    redis: web::Data<Arc<Mutex<MultiplexedConnection>>>,
) -> Result<HttpResponse, actix_web::error::Error> {

    let key = "lleo_key";

    async move || -> anyhow::Result<HttpResponse> {

        let mut conn = redis.lock().await;

        // read
        let result: Option<RedisArray> = redis_read(&mut *conn, key).await?;

        // save
        redis_save(&mut *conn, key, "new_value", 5).await?;

        let response = match result {
            Some(entry) => HttpResponse::Ok().json(entry),
            None => HttpResponse::NotFound().body("empty"),
        };

        Ok(response)
    }()
    .await
    .map_err(|err| {
        tracing::error!(error = %err, "Internal error in GET handler");
        actix_web::error::ErrorInternalServerError("internal error")
    })
}


/// put

pub async fn put(
    req: HttpRequest,
    path: ObjectPath,
    body: web::Bytes,
    redis: web::Data<Arc<Mutex<MultiplexedConnection>>>,
) -> Result<HttpResponse, actix_web::error::Error> {

    let (workspace, key) = path.into_inner();

//    println!("\nworkspace = {}", workspace);
//    println!("key = {}\n", key);

    trace!(workspace, key, "put request");

    async move || -> anyhow::Result<HttpResponse> {

        let mut conn = redis.lock().await;

/*
    let wsuuid = Uuid::parse_str(workspace.as_str()).map_err(|e| error::ErrorBadRequest(format!("Invalid UUID in workspace: {}", e)))?;

//        Headers: TTL or absolute expiration time
//            HULY-TTL
//            HULY-EXPIRE-AT
//        Conditional Headers
//            If-*

	let value = "new_value";

	// let new_md5 = md5::compute(&body);
        // save
*/

	let ttl = 5;
        redis_save(&mut *conn, key.as_str(), &body[..], ttl).await?;

        // Ok("response")
	return Ok(HttpResponse::Ok().body("DONE"));

    }()
    .await
    .map_err(|err| {
        tracing::error!(error = %err, "Internal error in GET handler");
        actix_web::error::ErrorInternalServerError("internal error")
    })
}



// delete

pub async fn delete(
    req: HttpRequest,
    path: ObjectPath,
    redis: web::Data<Arc<Mutex<MultiplexedConnection>>>,
) -> Result<HttpResponse, actix_web::error::Error> {

    let (workspace, key) = path.into_inner();
    trace!(workspace, key, "delete request");

    let wsuuid = Uuid::parse_str(workspace.as_str())
        .map_err(|e| error::ErrorBadRequest(format!("Invalid UUID in workspace: {}", e)))?;
    let keystr = key.as_str();
    // let key = "lleo_key";

    async move || -> anyhow::Result<HttpResponse> {
        let mut conn = redis.lock().await;

        let deleted = redis_delete(&mut *conn, keystr).await?;

        let response = match deleted {
            true => HttpResponse::NoContent().finish(),
            false => HttpResponse::NotFound().body("not found"),
        };

        Ok(response)
    }()
    .await
    .map_err(|err| {
        tracing::error!(error = %err, "Internal error in DELETE handler");
        actix_web::error::ErrorInternalServerError("internal error")
    })
}
