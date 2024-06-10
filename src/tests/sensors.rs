use actix_web::{test, web, App};
use sqlx::{Executor, PgPool};
use serde_json::json;
use crate::config;
use crate::routes::sensors::*;
use crate::models::{Sensor, SensorRequest, NewSensorMeasurementRequest};

// #[actix_rt::test]
// async fn test_get_all_sensors() {
//     let pool = setup_test_db().await;
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(sensors_routes)
//     ).await;
//
//     let req = test::TestRequest::get().uri("/sensors").to_request();
//     let resp = test::call_service(&app, req).await;
//
//     assert!(resp.status().is_success());
// }
//
// #[actix_rt::test]
// async fn test_get_sensor_by_id() {
//     let pool = setup_test_db().await;
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(sensors_routes)
//     ).await;
//
//     let req = test::TestRequest::get().uri("/sensors/1").to_request();
//     let resp = test::call_service(&app, req).await;
//
//     assert!(resp.status().is_success());
// }

// #[actix_rt::test]
// async fn test_create_sensor() {
//     let pool = setup_test_db().await;
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(sensors_routes)
//     ).await;
//
//     let new_sensor = SensorRequest {
//         name: String::from("new_sensor"),
//         sensors_measurements: vec![
//             NewSensorMeasurementRequest {
//                 type_id: 1,
//                 type_formula: Some(String::from("ax^2")),
//             },
//             NewSensorMeasurementRequest {
//                 type_id: 2,
//                 type_formula: None,
//             },
//         ],
//     };
//
//     let req = test::TestRequest::post()
//         .uri("/sensors")
//         .set_json(&new_sensor)
//         .to_request();
//     let resp = test::call_service(&app, req).await;
//
//     assert!(resp.status().is_success());
// }

// #[actix_rt::test]
// async fn test_update_sensor() {
//     let pool = setup_test_db().await;
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(sensors_routes)
//     ).await;
//
//     let update_sensor = UpdateSensorRequest {
//         name: Some(String::from("updated_sensor")),
//         sensors_measurements: None,
//     };
//
//     let req = test::TestRequest::put()
//         .uri("/sensors/1")
//         .set_json(&update_sensor)
//         .to_request();
//     let resp = test::call_service(&app, req).await;
//
//     assert!(resp.status().is_success());
// }

// #[actix_rt::test]
// async fn test_delete_sensor() {
//     let pool = setup_test_db().await;
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(sensors_routes)
//     ).await;
//
//     let req = test::TestRequest::delete().uri("/sensors/1").to_request();
//     let resp = test::call_service(&app, req).await;
//
//     assert!(resp.status().is_success());
// }

async fn setup_test_db() -> PgPool {
    let pool = config::get_db_pool().await.expect("Failed to create pool.");

    pool
}
