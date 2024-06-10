use sqlx::{PgPool, query, query_as, Row};
use crate::models::{SensorResponse, SensorMeasurementResponse, SensorRequest};

pub async fn fetch_sensors(pool: &PgPool) -> Result<Vec<SensorResponse>, sqlx::Error> {
    let sensors = query!("SELECT id as sensor_id, name as sensor_name FROM sensors")
        .fetch_all(pool)
        .await?;

    let mut sensor_responses = Vec::new();

    for sensor in sensors {
        let measurements = query!(
            "SELECT sm.type_id, mt.name as type_name, mt.units as type_units, sm.measurment_formula as type_formula
             FROM sensors_measurements sm
             JOIN measurements_type mt ON sm.type_id = mt.id
             WHERE sm.sensor_id = $1",
            sensor.sensor_id
        )
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| SensorMeasurementResponse {
                type_id: row.type_id,
                type_name: row.type_name.clone(),
                type_units: row.type_units.clone(),
                type_formula: row.type_formula.clone(),
            })
            .collect();

        sensor_responses.push(SensorResponse {
            sensor_id: sensor.sensor_id,
            sensor_name: sensor.sensor_name.clone(),
            sensors_measurements: measurements,
        });
    }

    Ok(sensor_responses)
}

pub async fn fetch_sensor(pool: &PgPool, sensor_id: i32) -> Result<SensorResponse, sqlx::Error> {
    let sensor = query!("SELECT id as sensor_id, name as sensor_name FROM sensors WHERE id = $1", sensor_id)
        .fetch_one(pool)
        .await?;

    let measurements = query!(
        "SELECT sm.type_id, mt.name as type_name, mt.units as type_units, sm.measurment_formula as type_formula
         FROM sensors_measurements sm
         JOIN measurements_type mt ON sm.type_id = mt.id
         WHERE sm.sensor_id = $1",
        sensor.sensor_id
    )
        .fetch_all(pool)
        .await?
        .iter()
        .map(|row| SensorMeasurementResponse {
            type_id: row.type_id,
            type_name: row.type_name.clone(),
            type_units: row.type_units.clone(),
            type_formula: row.type_formula.clone(),
        })
        .collect();

    Ok(SensorResponse {
        sensor_id: sensor.sensor_id,
        sensor_name: sensor.sensor_name,
        sensors_measurements: measurements,
    })
}

pub async fn fetch_sensor_types(pool: &PgPool, sensor_id: i32, ) -> Result<Vec<SensorMeasurementResponse>, sqlx::Error> {
    let rows = query!(
        "SELECT mt.id as type_id, mt.name as type_name, mt.units as type_units, sm.measurment_formula as type_formula
         FROM sensors_measurements sm
         JOIN measurements_type mt ON sm.type_id = mt.id
         WHERE sm.sensor_id = $1",
        sensor_id
    )
        .fetch_all(pool)
        .await?;

    let sensor_types: Vec<SensorMeasurementResponse> = rows
        .into_iter()
        .map(|row| SensorMeasurementResponse {
            type_id: row.type_id,
            type_name: row.type_name,
            type_units: row.type_units,
            type_formula: row.type_formula,
        })
        .collect();

    Ok(sensor_types)
}

pub async fn insert_sensor(pool: &PgPool, new_sensor: &SensorRequest, ) -> Result<SensorResponse, sqlx::Error> {
    let sensor_id: i32 = query("INSERT INTO sensors (name) VALUES ($1) RETURNING id")
        .bind(&new_sensor.sensor_name)
        .fetch_one(pool)
        .await?
        .try_get(0)?;

    for measurement in &new_sensor.sensors_measurements {
        let type_formula = measurement.type_formula.clone().unwrap_or_else(|| "value".to_string());

        query(
            "INSERT INTO sensors_measurements (sensor_id, type_id, measurment_formula) VALUES ($1, $2, $3)"
        )
            .bind(sensor_id)
            .bind(measurement.type_id)
            .bind(type_formula)
            .execute(pool)
            .await?;
    }

    let mut response_measurements = Vec::new();

    for measurement in &new_sensor.sensors_measurements {
        if let Ok(row) = query_as::<_, (String, String)>(
            "SELECT name, units FROM measurements_type WHERE id = $1"
        )
            .bind(measurement.type_id)
            .fetch_one(pool)
            .await
        {
            response_measurements.push(SensorMeasurementResponse {
                type_id: measurement.type_id,
                type_name: row.0,
                type_units: row.1,
                type_formula: measurement.type_formula.clone().unwrap_or_else(|| "value".to_string()),
            });
        }
    }

    Ok(SensorResponse {
        sensor_id,
        sensor_name: new_sensor.sensor_name.clone(),
        sensors_measurements: response_measurements,
    })
}

pub async fn update_one_sensor(pool: &PgPool, sensor_id: i32, update_sensor: &SensorRequest) -> Result<SensorResponse, sqlx::Error> {
    query("UPDATE sensors SET name = $1 WHERE id = $2")
        .bind(&update_sensor.sensor_name)
        .bind(sensor_id)
        .execute(pool)
        .await?;

    query("DELETE FROM sensors_measurements WHERE sensor_id = $1")
        .bind(sensor_id)
        .execute(pool)
        .await?;

    for measurement in &update_sensor.sensors_measurements {
        let type_formula = measurement.type_formula.clone().unwrap_or_else(|| "value".to_string());

        query(
            "INSERT INTO sensors_measurements (sensor_id, type_id, measurment_formula) VALUES ($1, $2, $3)"
        )
            .bind(sensor_id)
            .bind(measurement.type_id)
            .bind(type_formula)
            .execute(pool)
            .await?;
    }

    fetch_sensor(pool, sensor_id).await
}

pub async fn delete_one_sensor(pool: &PgPool, sensor_id: i32) -> Result<(), sqlx::Error> {
    let count: Option<i64> = query!("SELECT COUNT(*) as count FROM meteostations_sensors WHERE sensor_id = $1", sensor_id)
        .fetch_one(pool)
        .await?
        .count;

    if count > Some(0) {
        return Err(sqlx::Error::RowNotFound);
    }

    query!("DELETE FROM sensors_measurements WHERE sensor_id = $1", sensor_id)
        .execute(pool)
        .await?;

    query!("DELETE FROM sensors WHERE id = $1", sensor_id)
        .execute(pool)
        .await?;

    Ok(())
}