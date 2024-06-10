use std::collections::HashMap;
use chrono::{Utc};
use sqlx::{PgPool, query};
use crate::models::{MeteostationResponse, MeteostationSensorResponse, MeteostationSensorCreateRequest, MeteostationSensorRemove};

pub async fn fetch_meteostation_sensors(
    pool: &PgPool,
) -> Result<Vec<MeteostationResponse>, sqlx::Error> {
    let meteostations = query!(
        "SELECT m.id AS station_id, m.name AS station_name, m.longitude AS station_longitude, m.latitude AS station_latitude,
                ms.inventory_number, ms.sensor_id, s.name AS sensor_name, ms.added_ts AS sensor_added_ts, ms.removed_ts AS sensor_remove_ts
         FROM meteostations m
         JOIN meteostations_sensors ms ON m.id = ms.station_id
         JOIN sensors s ON ms.sensor_id = s.id"
    )
        .fetch_all(pool)
        .await?;

    let mut stations_map: HashMap<i32, MeteostationResponse> = HashMap::new();

    for record in meteostations {
        let sensor = MeteostationSensorResponse {
            sensor_inventory_number: record.inventory_number,
            sensor_id: record.sensor_id,
            sensor_name: record.sensor_name,
            sensor_added_ts: record.sensor_added_ts,
            sensor_remove_ts: record.sensor_remove_ts,
        };

        stations_map
            .entry(record.station_id)
            .or_insert(MeteostationResponse {
                station_id: record.station_id,
                station_name: record.station_name,
                station_longitude: record.station_longitude,
                station_latitude: record.station_latitude,
                sensors: vec![],
            })
            .sensors
            .push(sensor);
    }

    let response: Vec<MeteostationResponse> = stations_map.into_values().collect();

    Ok(response)
}

pub async fn insert_meteostation_sensors(
    pool: &PgPool,
    item: &MeteostationSensorCreateRequest
) -> Result<(), sqlx::Error> {

    for sensor in &item.meteostations_sensors {
        query!(
            "INSERT INTO meteostations_sensors (station_id, sensor_id, added_ts) VALUES ($1, $2, $3)",
            sensor.station_id,
            sensor.sensor_id,
            sensor.added_ts.unwrap_or_else(|| Utc::now().naive_utc())
        )
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn remove_meteostation_sensor(
    pool: &PgPool,
    number: String,
    item: &MeteostationSensorRemove
) -> Result<(), sqlx::Error> {

    query!(
        "UPDATE meteostations_sensors SET removed_ts = $1 WHERE inventory_number = $2",
        item.removed_ts.unwrap_or_else(|| Utc::now().naive_utc()),
        number
    )
        .execute(pool)
        .await?;

    Ok(())
}