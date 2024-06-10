use actix_web::{
    web, HttpResponse, Responder,
    get, post, delete
};
use sqlx::PgPool;

use crate::handlers::measurements::*;
use crate::models::{MeasurementQuery, MeasurementRequest};

#[utoipa::path(
    get,
    path = "/api/measurements",
    responses(
        (status = 200, description = "Get all measurements", body = [Measurement])
    )
)]
#[get("/api/measurements")]
pub async fn get_measurements(pool: web::Data<PgPool>) -> impl Responder {
    match fetch_all_measurements(pool.get_ref()).await {
        Ok(measurements) => HttpResponse::Ok().json(measurements),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/measurements",
    params(
        ("meteostation" = Option<i32>, Query, description = "Meteostation ID"),
        ("sensor" = Option<i32>, Query, description = "Sensor ID")
    ),
    responses(
        (status = 200, description = "Get all measurements by condition", body = [Measurement])
    )
)]
#[get("/api/measurements")]
pub async fn get_condition_measurements(pool: web::Data<PgPool>, query: web::Query<MeasurementQuery>) -> impl Responder {
    match fetch_condition_measurements(pool.get_ref(), query).await {
        Ok(measurements) => HttpResponse::Ok().json(measurements),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    post,
    path = "/api/measurements",
    request_body = MeasurementRequest,
    responses(
        (status = 200, description = "Create measurements")
    )
)]
#[post("/api/measurements")]
pub async fn create_measurements(pool: web::Data<PgPool>, item: web::Json<MeasurementRequest>) -> impl Responder {
    match insert_measurements(pool.get_ref(), &item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[utoipa::path(
    delete,
    path = "/api/measurements/{sensor_inventory_number}",
    params(
        ("sensor_inventory_number" = i32, description = "sensor_inventory_number")
    ),
    responses(
        (status = 200, description = "Delete measurement")
    )
)]
#[delete("/api/measurements/{sensor_inventory_number}")]
pub async fn remove_measurement(pool: web::Data<PgPool>, number: web::Path<String>) -> impl Responder {
    match delete_measurement(pool.get_ref(), number.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


pub fn measurements_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_measurements);
    cfg.service(get_condition_measurements);
    cfg.service(create_measurements);
    cfg.service(remove_measurement);
}