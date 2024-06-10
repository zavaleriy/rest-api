use actix_web::{
    web, HttpResponse, Responder,
    get, post, put, delete
};
use sqlx::PgPool;

use crate::handlers::sensors::*;
use crate::models::*;

#[utoipa::path(
get,
path = "/api/sensors",
responses(
(status = 200, description = "Get all sensors", body = [SensorResponse])
)
)]
#[get("/api/sensors")]
async fn get_sensors(pool: web::Data<PgPool>) -> impl Responder {
    match fetch_sensors(pool.get_ref()).await {
        Ok(sensors) => HttpResponse::Ok().json(sensors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
get,
path = "/api/sensors/{id}",
params(
("id" = i32, description = "Sensor ID")
),
responses(
(status = 200, description = "Get sensor by ID", body = SensorResponse)
)
)]
#[get("/api/sensors/{id}")]
async fn get_sensor(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match fetch_sensor(pool.get_ref(), path.into_inner()).await {
        Ok(sensor) => HttpResponse::Ok().json(sensor),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
get,
path = "/api/sensors/{id}/type",
params(
("id" = i32, description = "Sensor ID")
),
responses(
(status = 200, description = "Get sensor types by ID", body = [SensorMeasurementResponse])
)
)]
#[get("/api/sensors/{id}/type")]
async fn get_sensor_types(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match fetch_sensor_types(pool.get_ref(), path.into_inner()).await {
        Ok(sensor_types) => HttpResponse::Ok().json(sensor_types),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
post,
path = "/api/sensors",
request_body = SensorRequest,
responses(
(status = 201, description = "Create new sensor", body = SensorResponse)
)
)]
#[post("/api/sensors")]
async fn create_sensor(pool: web::Data<PgPool>, sensor: web::Json<SensorRequest>) -> impl Responder {
    match insert_sensor(pool.get_ref(), &sensor.into_inner()).await {
        Ok(new_sensor) => HttpResponse::Created().json(new_sensor),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
put,
path = "/api/sensors/{id}",
params(
("id" = i32, description = "Sensor ID")
),
request_body = SensorRequest,
responses(
(status = 200, description = "Update sensor", body = SensorResponse)
)
)]
#[put("/api/sensors/{id}")]
async fn update_sensor(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    sensor: web::Json<SensorRequest>,
) -> impl Responder {
    match update_one_sensor(pool.get_ref(), path.into_inner(), &sensor.into_inner()).await {
        Ok(updated_sensor) => HttpResponse::Ok().json(updated_sensor),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
delete,
path = "/api/sensors/{id}",
params(
("id" = i32, description = "Sensor ID")
),
responses(
(status = 200, description = "Delete sensor"),
(status = 404, description = "Sensor not found or has measurements in meteostations_sensors")
)
)]
#[delete("/api/sensors/{id}")]
async fn delete_sensor(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match delete_one_sensor(pool.get_ref(), path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn sensors_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_sensors);
    cfg.service(get_sensor);
    cfg.service(get_sensor_types);
    cfg.service(create_sensor);
    cfg.service(update_sensor);
    cfg.service(delete_sensor);
}