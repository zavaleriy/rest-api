mod models;
mod routes;
use routes::*;
mod handlers;
mod config;
mod tests;

use std::env;
use actix_web::{App, HttpServer, web, middleware::Logger};
use dotenv::dotenv;
use actix_cors::Cors;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        models::Sensor,
        models::Measurement,
        models::MeteostationSensor,
        models::SensorMeasurement,
        models::MeasurementType,
        models::Meteostation,

        models::SensorRequest,
        models::SensorResponse,
        models::MeteostationRequest,
        models::MeasurementTypeRequest,
        models::SensorMeasurementRequest,
        models::SensorMeasurementsDelete,
        models::SensorMeasurementCreate,
        models::SensorMeasurementResponse,
        models::NewSensorMeasurementRequest,
        models::MeteostationResponse,
        models::MeteostationSensorResponse,
        models::MeteostationSensorCreate,
        models::MeteostationSensorCreateRequest,
        models::MeteostationSensorRemove,
        models::MeasurementRequest,

        BigDecimal,
    )),
    paths(
        sensors::get_sensors,
        sensors::get_sensor,
        sensors::get_sensor_types,
        sensors::create_sensor,
        sensors::update_sensor,
        sensors::delete_sensor,

        meteostations::get_all_meteostations,
        meteostations::get_sensor_meteostation,
        meteostations::create_meteostation,
        meteostations::update_meteostation,
        meteostations::delete_meteostation,

        measurement_type::get_all_measurement_types,
        measurement_type::create_measurement_type,
        measurement_type::update_measurement_type,
        measurement_type::delete_measurement_type,

        sensors_measurements::create_sensor_measurements,
        sensors_measurements::delete_sensor_measurements,

        meteostations_sensor::get_all_meteostations_sensor,
        meteostations_sensor::create_meteostations_sensor,
        meteostations_sensor::delete_meteostation_sensor,

        measurements::get_measurements,
        measurements::get_condition_measurements,
        measurements::create_measurements,
        measurements::remove_measurement,
    )
)]
struct ApiDoc;

#[allow(dead_code)]
#[derive(ToSchema)]
struct BigDecimal(f64);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let pool = config::get_db_pool().await.expect("Failed to create pool.");

    let openapi = ApiDoc::openapi();

    println!("Server is running on http://localhost:8000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(sensors_routes)
            .configure(measurement_type_routes)
            .configure(meteostations_routes)
            .configure(sensor_measurements_routes)
            .configure(meteostations_sensor_routes)
            .configure(measurements_routes)
            .service(SwaggerUi::new("/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;

    Ok(())
}