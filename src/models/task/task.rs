#![allow(clippy::redundant_static_lifetimes, clippy::needless_return)]
use super::Priority;
use crate::db::schema::tasks;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub details: Option<String>,
    #[serde(with = "iso_timestamp")]
    pub created_timestamp: NaiveDateTime,
    #[serde(with = "nullable_iso_timestamp")]
    pub updated_timestamp: Option<NaiveDateTime>,
    #[serde(with = "nullable_iso_timestamp")]
    pub deadline: Option<NaiveDateTime>,
    #[serde(with = "nullable_iso_timestamp")]
    pub completed_timestamp: Option<NaiveDateTime>,
    pub priority: Priority,
    pub persistent: bool,
    pub supertask: Option<Uuid>,
}

#[derive(Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[table_name = "tasks"]
pub struct OptionalizedTask {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub details: Option<String>,
    #[serde(with = "nullable_iso_timestamp")]
    #[serde(default)]
    pub created_timestamp: Option<NaiveDateTime>,
    #[serde(with = "nullable_iso_timestamp")]
    #[serde(default)]
    pub updated_timestamp: Option<NaiveDateTime>,
    #[serde(with = "nullable_iso_timestamp")]
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    #[serde(with = "nullable_iso_timestamp")]
    #[serde(default)]
    pub completed_timestamp: Option<NaiveDateTime>,
    pub priority: Option<Priority>,
    pub persistent: Option<bool>,
    pub supertask: Option<Uuid>,
}

impl From<Task> for OptionalizedTask {
    fn from(task: Task) -> Self {
        return OptionalizedTask {
            id: Some(task.id),
            title: Some(task.title),
            details: task.details,
            created_timestamp: Some(task.created_timestamp),
            updated_timestamp: task.updated_timestamp,
            deadline: task.deadline,
            priority: Some(task.priority),
            persistent: Some(task.persistent),
            completed_timestamp: task.completed_timestamp,
            supertask: task.supertask,
        };
    }
}

impl PartialEq<OptionalizedTask> for Task {
    fn eq(&self, other: &OptionalizedTask) -> bool {
        let optionalized_self = OptionalizedTask::from(self.clone());
        return &optionalized_self == other;
    }
}

impl PartialEq<Task> for OptionalizedTask {
    fn eq(&self, other: &Task) -> bool {
        let optionalized_other = OptionalizedTask::from(other.clone());
        return &optionalized_other == self;
    }
}

mod iso_timestamp {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        return serializer.serialize_str(&s);
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        return Ok(NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?);
    }
}

mod nullable_iso_timestamp {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
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
