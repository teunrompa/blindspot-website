#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub instagram_url: Option<String>,
    pub spotify_url: Option<String>,
    pub photo_url: Option<String>,
}
