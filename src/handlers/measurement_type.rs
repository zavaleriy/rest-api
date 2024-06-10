use sqlx::{PgPool, query, query_as, Row};
use crate::models::{MeasurementType, MeasurementTypeRequest};

pub async fn fetch_measurement_types(pool: &PgPool) -> Result<Vec<MeasurementType>, sqlx::Error> {
    let rows = query_as!(
        MeasurementType,
        "SELECT id, name, units FROM measurements_type"
    )
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn insert_measurement_type(pool: &PgPool, mtype: &MeasurementTypeRequest) -> Result<MeasurementType, sqlx::Error> {
    let mtype_id = query(
        "INSERT INTO measurements_type (name, units) VALUES ($1, $2) RETURNING id"
    )
        .bind(&mtype.name)
        .bind(&mtype.units)
        .fetch_one(pool)
        .await?
        .try_get(0)?;

    Ok(MeasurementType {
        id: mtype_id,
        name: mtype.name.clone(),
        units: mtype.units.clone()
    })
}

pub async fn update_one_measurement_type(
    pool: &PgPool,
    type_id: i32,
    item: &MeasurementTypeRequest,
) -> Result<MeasurementType, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE measurements_type
        SET name = COALESCE($1, name),
            units = COALESCE($2, units)
        WHERE id = $3
        RETURNING id, name, units
        "#,
        item.name,
        item.units,
        type_id
    )
        .fetch_one(pool)
        .await?;

    Ok(MeasurementType {
        id: result.id,
        name: result.name,
        units: result.units
    })
}

pub async fn delete_one_measurement_type(pool: &PgPool, type_id: i32) -> Result<(), sqlx::Error> {
    let count = sqlx::query!(
        "SELECT COUNT(*) FROM sensors_measurements WHERE type_id = $1",
        type_id
    )
        .fetch_one(pool)
        .await?
        .count;

    if count > Some(0) {
        return Err(sqlx::Error::RowNotFound);
    }

    query!(
        "DELETE FROM measurements_type WHERE id = $1",
        type_id
    )
        .execute(pool)
        .await?;

    Ok(())
}