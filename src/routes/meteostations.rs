use actix_web::{get, web, HttpResponse, Responder, post, put, delete};
use crate::handlers::meteostations::*;
use sqlx::PgPool;
use crate::models::MeteostationRequest;

#[utoipa::path(
get,
path = "/api/meteostations",
responses(
(status = 200, description = "Get all meteostations", body = [Meteostation])
)
)]
#[get("/api/meteostations")]
async fn get_all_meteostations(pool: web::Data<PgPool>) -> impl Responder {
    match fetch_meteostations(pool.get_ref()).await {
        Ok(meteostations) => HttpResponse::Ok().json(meteostations),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
get,
path = "/api/meteostations/{station_id}/sensor",
responses(
(status = 200, description = "Get all sensors of meteostation", body = [Sensor])
)
)]
#[get("/api/meteostations/{station_id}/sensor")]
async fn get_sensor_meteostation(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match fetch_sensor_meteostation(pool.get_ref(), path.into_inner()).await {
        Ok(meteostation) => HttpResponse::Ok().json(meteostation),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
post,
path = "/api/meteostations",
request_body = MeteostationRequest,
responses(
(status = 201, description = "Create new meteostation", body = Meteostation)
)
)]
#[post("/api/meteostations")]
async fn create_meteostation(
    pool: web::Data<PgPool>,
    meteostation: web::Json<MeteostationRequest>
) -> impl Responder {
    match insert_meteostation(pool.get_ref(), &meteostation.into_inner()).await {
        Ok(meteostation) => HttpResponse::Created().json(meteostation),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
put,
path = "/api/meteostations/{id}",
params(
("id" = i32, description = "Station ID")
),
request_body = MeteostationRequest,
responses(
(status = 200, description = "Update station", body = Meteostation)
)
)]
#[put("/api/meteostations/{id}")]
async fn update_meteostation(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    station: web::Json<MeteostationRequest>
) -> impl Responder {
    match update_one_station(pool.get_ref(), path.into_inner(), &station.into_inner()).await {
        Ok(station) => HttpResponse::Ok().json(station),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[utoipa::path(
delete,
path = "/api/meteostations/{id}",
params(
("id" = i32, description = "Meteostation ID")
),
responses(
(status = 200, description = "Delete meteostation"),
(status = 404, description = "Meteostation not found or has measurements in meteostations_sensors")
)
)]
#[delete("/api/meteostations/{id}")]
async fn delete_meteostation(
    pool: web::Data<PgPool>,
    path: web::Path<i32>
) -> impl Responder {
    match delete_one_station(pool.get_ref(), path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn meteostations_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_meteostations);
    cfg.service(get_sensor_meteostation);
    cfg.service(create_meteostation);
    cfg.service(update_meteostation);
    cfg.service(delete_meteostation);
}
