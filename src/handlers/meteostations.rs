use sqlx::{PgPool, query, query_as, Row};
use crate::models::*;

pub async fn fetch_meteostations(pool: &PgPool) -> Result<Vec<Meteostation>, sqlx::Error> {
    let rows = query_as!(
        Meteostation,
        "SELECT id, name, longitude, latitude FROM meteostations"
    )
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn fetch_meteostation(pool: &PgPool, station_id: i32) -> Result<Meteostation, sqlx::Error> {
    let station = query_as!(
        Meteostation,
        r#"
        SELECT id, name, longitude, latitude
        FROM meteostations
        WHERE id = $1
        "#, station_id
    )
        .fetch_one(pool)
        .await?;

    Ok(Meteostation {
        id: station.id,
        name: station.name,
        longitude: station.longitude,
        latitude: station.latitude
    })
}

pub async fn fetch_sensor_meteostation(pool: &PgPool, station_id: i32) -> Result<Vec<Sensor>, sqlx::Error> {

    let sensors = query_as!(
        Sensor,
        r#"
        SELECT DISTINCT s.id, s.name
        FROM sensors s
        JOIN meteostations_sensors ms ON s.id = ms.sensor_id
        WHERE ms.station_id = $1
        ORDER BY s.id
        "#, station_id
    )
        .fetch_all(pool)
        .await?;

    Ok(sensors)
}

pub async fn insert_meteostation(pool: &PgPool, station: &MeteostationRequest) -> Result<Meteostation, sqlx::Error> {
    let station_id = sqlx::query(
        "INSERT INTO meteostations (name, longitude, latitude) VALUES ($1, $2, $3) RETURNING id")
        .bind(&station.name)
        .bind(&station.longitude)
        .bind(&station.latitude)
        .fetch_one(pool)
        .await?
        .try_get(0)?;

    Ok(Meteostation {
        id: station_id,
        name: station.name.clone(),
        longitude: station.longitude.clone(),
        latitude: station.latitude.clone()
    })
}

pub async fn update_one_station(pool: &PgPool, station_id: i32, station: &MeteostationRequest) -> Result<Meteostation, sqlx::Error> {
    query_as!(
        Meteostation,
        r#"
        UPDATE meteostations
        SET name = COALESCE($1, name),
            longitude = COALESCE($2, longitude),
            latitude = COALESCE($3, latitude)
        WHERE id = $4
        RETURNING id, name, longitude, latitude
        "#,
        station.name,
        station.longitude,
        station.latitude,
        station_id
    )
        .fetch_one(pool)
        .await?;

    fetch_meteostation(pool, station_id).await
}

pub async fn delete_one_station(pool: &PgPool, station_id: i32) -> Result<(), sqlx::Error> {
    let count: Option<i64> = query!("SELECT COUNT(*) as count FROM meteostations_sensors WHERE station_id = $1", station_id)
        .fetch_one(pool)
        .await?
        .count;

    if count > Some(0) {
        return Err(sqlx::Error::RowNotFound);
    }

    query!("DELETE FROM meteostations WHERE id = $1", station_id)
        .execute(pool)
        .await?;

    Ok(())

}