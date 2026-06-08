use crate::stored_types::artist::Artist;

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "event_status", rename_all = "snake_case")]
pub enum EventStatus {
    SoldOut,
    Available,
    ComingSoon,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub venue: String,
    pub status: EventStatus,
}

#[derive(Debug, serde::Serialize)]
pub struct EventWithLineup {
    pub id: i32,
    pub name: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub venue: String,
    pub status: EventStatus,
    pub lineup: Vec<Artist>,
}
