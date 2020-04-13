use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::{Queryable, Identifiable, Insertable};
use crate::db::schema::tasks;
use super::Priority;


#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name="tasks"]
pub struct Task {
  pub id: Uuid,
  pub title: String,
  pub details: Option<String>,
  #[serde(with = "iso_timestamp")]
  pub created_date: NaiveDateTime,
  #[serde(with = "nullable_iso_timestamp")]
  pub deadline: Option<NaiveDateTime>,
  pub priority: Priority,
  pub persistent: bool,
  pub completed: bool,
  pub supertask: Option<Uuid>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name="tasks"]
pub struct NewTask<'a> {
  pub title: &'a str,
  pub details: Option<&'a str>,
  #[serde(with = "nullable_iso_timestamp")]
  #[serde(default)]
  pub deadline: Option<NaiveDateTime>,
  pub priority: Option<Priority>,
  pub persistent: Option<bool>,
  pub completed: Option<bool>,
  pub supertask: Option<Uuid>,
}


mod iso_timestamp {
  use chrono::{NaiveDateTime, Utc, TimeZone};
  use serde::{self, Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

  pub fn serialize<S>(
    date: &NaiveDateTime,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!("{}", date.format(FORMAT));
    return serializer.serialize_str(&s);
  }

  pub fn deserialize<'de, D>(
    deserializer: D,
  ) -> Result<NaiveDateTime, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    return Ok(
      NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?
    );
  }
}

mod nullable_iso_timestamp {
  use chrono::{NaiveDateTime, Utc, TimeZone};
  use serde::{self, Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

  pub fn serialize<S>(
    date: &Option<NaiveDateTime>, 
    serializer: S
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if let Some(ref d) = *date {
      let s = &d.format(FORMAT).to_string();
      return serializer.serialize_str(&s);
    } else {
      return serializer.serialize_none();
    }
  }

  pub fn deserialize<'de, D>(
    deserializer: D
  ) -> Result<Option<NaiveDateTime>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
      return Ok(Some(
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?,
      ));
    } else {
      return Ok(None);
    }
  }
}