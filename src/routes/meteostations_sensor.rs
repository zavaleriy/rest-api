use actix_web::{
    web, HttpResponse, Responder,
    get, post, put
};
use sqlx::PgPool;

use crate::handlers::meteostations_sensor::*;
use crate::models::{MeteostationSensorCreateRequest, MeteostationSensorRemove};

#[utoipa::path(
    get,
    path = "/api/meteostations_sensors",
    responses(
        (status = 200, description = "Get all meteostation sensors", body = [MeteostationResponse])
    )
)]
#[get("/api/meteostations_sensors")]
pub async fn get_all_meteostations_sensor(pool: web::Data<PgPool>) -> impl Responder {
    match fetch_meteostation_sensors(pool.get_ref()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[utoipa::path(
    post,
    path = "/api/meteostations_sensors",
    responses(
        (status = 200, description = "Create binding of sensors to stations")
    )
)]
#[post("/api/meteostations_sensors")]
pub async fn create_meteostations_sensor(pool: web::Data<PgPool>, item: web::Json<MeteostationSensorCreateRequest>) -> impl Responder {
    match insert_meteostation_sensors(pool.get_ref(), &item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[utoipa::path(
    put,
    path = "/api/meteostations_sensors/{inventory_number}/removed_ts",
    request_body = MeteostationSensorRemove,
    responses(
        (status = 200, description = "Meteostation sensor removed"),
        (status = 400, description = "Bad request")
    )
)]
#[put("/api/meteostations_sensors/{inventory_number}/removed_ts")]
pub async fn delete_meteostation_sensor(pool: web::Data<PgPool>, number: web::Path<String>, item: web::Json<MeteostationSensorRemove>) -> impl Responder {
    match remove_meteostation_sensor(pool.get_ref(), number.into_inner(), &item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub fn meteostations_sensor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_meteostations_sensor);
    cfg.service(create_meteostations_sensor);
    cfg.service(delete_meteostation_sensor);
}