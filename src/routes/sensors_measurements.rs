use actix_web::{web, HttpResponse, Responder, post, delete};
use crate::handlers::sensors_measurements::*;
use sqlx::PgPool;
use crate::models::{SensorMeasurementRequest, SensorMeasurementsDelete};

#[utoipa::path(
    post,
    path = "/api/sensor_measurements/{sensor_id}",
    params(
        ("sensor_id" = i32, description = "Sensor ID")
    ),
    request_body = SensorMeasurementRequest,
    responses(
    (status = 201, description = "Create new sensor measurements")
    )
)]
#[post("/api/sensor_measurements/{sensor_id}")]
async fn create_sensor_measurements(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    sensor_measurement: web::Json<SensorMeasurementRequest>
) -> impl Responder {
    match insert_sensor_measurements(pool.get_ref(), id.into_inner(), &sensor_measurement.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/sensor_measurements/{sensor_id}",
    params(
        ("sensor_id" = i32, description = "Sensor ID")
    ),
    request_body = SensorMeasurementsDelete,
    responses(
        (status = 200, description = "Deleted sensor measurements"),
    )
)]
#[delete("/api/sensor_measurements/{sensor_id}")]
async fn delete_sensor_measurements(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    path: web::Json<SensorMeasurementsDelete>
) -> impl Responder {
    match delete_many_sensor_measurements(pool.get_ref(), id.into_inner(), &path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn sensor_measurements_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_sensor_measurements);
    cfg.service(delete_sensor_measurements);
}