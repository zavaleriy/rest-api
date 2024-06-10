use sqlx::{PgPool};
use crate::models::{SensorMeasurementRequest, SensorMeasurementsDelete};

pub async fn insert_sensor_measurements(
    pool: &PgPool,
    sensor_id: i32,
    item: &SensorMeasurementRequest,
) -> Result<(), sqlx::Error> {
    for measurement in &item.sensors_measurements {
        sqlx::query!(
            "INSERT INTO sensors_measurements (sensor_id, type_id, measurment_formula) VALUES ($1, $2, $3)",
            sensor_id,
            measurement.type_id,
            measurement.measurement_formula
        )
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn delete_many_sensor_measurements(
    pool: &PgPool,
    sensor_id: i32,
    item: &SensorMeasurementsDelete,
) -> Result<(), sqlx::Error> {
    let sensor_id = sensor_id;

    for type_id in &item.measurements_type {
        sqlx::query!(
            "DELETE FROM sensors_measurements WHERE sensor_id = $1 AND type_id = $2",
            sensor_id,
            type_id
        )
            .execute(pool)
            .await?;
    }

    Ok(())
}