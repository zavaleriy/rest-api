use actix_web::web;
use sqlx::{PgPool, query, query_as};
use crate::models::{Measurement, MeasurementRequest, MeasurementQuery};

pub async fn fetch_all_measurements(pool: &PgPool) -> Result<Vec<Measurement>, sqlx::Error> {

    let measurements = query_as!(
        Measurement,
        "SELECT sensor_inventory_number, value, ts, type FROM measurements"
    )
        .fetch_all(pool)
        .await?;

    Ok(measurements)

}

pub async fn fetch_condition_measurements(pool: &PgPool, query: web::Query<MeasurementQuery>) -> Result<Vec<Measurement>, sqlx::Error> {

    let mut sql = String::from("SELECT * FROM measurements JOIN meteostations_sensors ON sensor_inventory_number = inventory_number WHERE 1 = 1");
    if let Some(station_id) = query.meteostation {
        sql.push_str(&format!(" AND station_id = {}", station_id));
    }
    if let Some(sensor_id) = query.sensor {
        sql.push_str(&format!(" AND sensor_id = {}", sensor_id));
    }

    let measurements = sqlx::query_as::<_, Measurement>(&sql)
        .fetch_all(pool)
        .await?;

    Ok(measurements)
}

pub async fn insert_measurements(pool: &PgPool, item: &MeasurementRequest) -> Result<(), sqlx::Error> {
    for measurement in &item.measurements {
       query!("INSERT INTO measurements (sensor_inventory_number, value, ts, type) VALUES ($1, $2, $3, $4)",
           measurement.sensor_inventory_number,
           measurement.value,
           measurement.ts,
           measurement.r#type
       )
           .execute(pool)
           .await?;
    }

    Ok(())
}

pub async fn delete_measurement(pool: &PgPool, number: String) -> Result<(), sqlx::Error> {

    query!("DELETE FROM measurements WHERE sensor_inventory_number = $1", number)
        .execute(pool)
        .await?;

    Ok(())
}