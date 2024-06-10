use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use chrono::{NaiveDateTime};
use utoipa::ToSchema;

mod datetime_format {
    use chrono::{NaiveDateTime, DateTime};
    use serde::{self, Serializer, Deserializer, Deserialize};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = DateTime::parse_from_rfc3339(&s)
            .map_err(serde::de::Error::custom)?
            .naive_utc();
        Ok(dt)
    }

    pub mod option {
        use super::*;
        use serde::{self, Serializer, Deserializer};

        pub fn serialize<S>(
            date: &Option<NaiveDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            match date {
                Some(d) => {
                    let s = d.format(FORMAT).to_string();
                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<Option<NaiveDateTime>, D::Error>
            where
                D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;
            match s {
                Some(s) => {
                    let dt = DateTime::parse_from_rfc3339(&s)
                        .map_err(serde::de::Error::custom)?
                        .naive_utc();
                    Ok(Some(dt))
                }
                None => Ok(None),
            }
        }
    }
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Sensor {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Measurement {
    pub sensor_inventory_number: String,
    pub value: BigDecimal,
    #[serde(with = "datetime_format")]
    pub ts: NaiveDateTime,
    pub r#type: Option<i32>,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct MeteostationSensor {
    pub inventory_number: String,
    pub station_id: i32,
    pub sensor_id: i32,
    #[serde(with = "datetime_format")]
    pub added_ts: NaiveDateTime,
    #[serde(with = "datetime_format::option")]
    pub removed_ts: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct SensorMeasurement {
    pub sensor_id: i32,
    pub type_id: i32,
    pub measurement_formula: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct MeasurementType {
    pub id: i32,
    pub name: String,
    pub units: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Meteostation {
    pub id: i32,
    pub name: String,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SensorRequest {
    pub sensor_name: String,
    pub sensors_measurements: Vec<NewSensorMeasurementRequest>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewSensorMeasurementRequest {
    pub type_id: i32,
    pub type_formula: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SensorMeasurementResponse {
    pub type_id: i32,
    pub type_name: String,
    pub type_units: String,
    pub type_formula: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SensorResponse {
    pub sensor_id: i32,
    pub sensor_name: String,
    pub sensors_measurements: Vec<SensorMeasurementResponse>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeteostationRequest {
    pub name: String,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeasurementTypeRequest {
    pub name: String,
    pub units: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SensorMeasurementCreate {
    pub type_id: i32,
    pub measurement_formula: Option<String>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SensorMeasurementRequest {
    pub sensors_measurements: Vec<SensorMeasurementCreate>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SensorMeasurementsDelete {
    pub measurements_type: Vec<i32>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeteostationSensorResponse {
    pub sensor_inventory_number: String,
    pub sensor_id: i32,
    pub sensor_name: String,
    #[serde(with = "datetime_format::option")]
    pub sensor_added_ts: Option<NaiveDateTime>,
    #[serde(with = "datetime_format::option")]
    pub sensor_remove_ts: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeteostationResponse {
    pub station_id: i32,
    pub station_name: String,
    pub station_longitude: BigDecimal,
    pub station_latitude: BigDecimal,
    pub sensors: Vec<MeteostationSensorResponse>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeteostationSensorCreate {
    pub station_id: i32,
    pub sensor_id: i32,
    #[serde(with = "datetime_format::option")]
    pub added_ts: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeteostationSensorCreateRequest {
    pub meteostations_sensors: Vec<MeteostationSensorCreate>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeteostationSensorRemove {
    #[serde(with = "datetime_format::option")]
    pub removed_ts: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeasurementRequest {
    pub measurements: Vec<Measurement>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeasurementQuery {
    pub meteostation: Option<i32>,
    pub sensor: Option<i32>,
}