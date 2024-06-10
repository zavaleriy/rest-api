use actix_web::{get, web, HttpResponse, Responder, post, put, delete};
use crate::handlers::measurement_type::*;
use sqlx::PgPool;
use crate::models::MeasurementTypeRequest;

#[utoipa::path(
get,
path = "/api/measurement_types",
responses(
(status = 200, description = "Get all measurement types", body = [MeasurementType])
)
)]
#[get("/api/measurement_types")]
async fn get_all_measurement_types(pool: web::Data<PgPool>) -> impl Responder {
    match fetch_measurement_types(pool.get_ref()).await {
        Ok(types) => HttpResponse::Ok().json(types),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    post,
    path = "/api/measurement_types",
    request_body = MeasurementTypeRequest,
    responses(
    (status = 201, description = "Create new measurement type", body = MeasurementType)
    )
)]
#[post("/api/measurement_types")]
async fn create_measurement_type(
    pool: web::Data<PgPool>,
    mtype: web::Json<MeasurementTypeRequest>
) -> impl Responder {
    match insert_measurement_type(pool.get_ref(), &mtype.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    put,
    path = "/api/measurement_types/{id}",
    params(
    ("id" = i32, description = "Measurement type ID")
    ),
    request_body = MeasurementTypeRequest,
    responses(
    (status = 200, description = "Update measurement type", body = MeasurementType)
    )
)]
#[put("/api/measurement_types/{id}")]
async fn update_measurement_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    mtype: web::Json<MeasurementTypeRequest>
) -> impl Responder {
    match update_one_measurement_type(pool.get_ref(), path.into_inner(), &mtype.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[utoipa::path(
    delete,
    path = "/api/measurement_types/{id}",
    params(
        ("id" = i32, description = "Measurement type ID")
    ),
    responses(
        (status = 200, description = "Delete measurement type"),
        (status = 404, description = "measurement type not found or used in meteostations_sensors")
    )
)]
#[delete("/api/measurement_types/{id}")]
async fn delete_measurement_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>
) -> impl Responder {
    match delete_one_measurement_type(pool.get_ref(), path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn measurement_type_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_measurement_types);
    cfg.service(create_measurement_type);
    cfg.service(update_measurement_type);
    cfg.service(delete_measurement_type);
}
