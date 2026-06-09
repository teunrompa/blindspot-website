mod stored_types;

use stored_types::artist::Artist;
use stored_types::events::EventWithLineup;
use stored_types::user::User;

use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    routing::get,
};

use sqlx::PgPool;

use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing::info;

use crate::stored_types::events::{self, Event};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("server=debug,tower_http=debug")
        .init();

    //GET db url from .env
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    //Setup routing & seriveces
    let app = Router::new()
        .route("/api/users/{id}", get(get_user))
        .route("/api/events", get(get_events))
        .route("/api/events{id}", get(get_event))
        .route("/api/events/{id}/lineup", get(get_lineup_for_event))
        .with_state(pool)
        .fallback_service(
            ServeDir::new("frontend/dist")
                .not_found_service(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = "0.0.0.0:3000";
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Listening on http://{addr}");
    axum::serve(listner, app).await.unwrap();
}

#[debug_handler]
async fn get_events(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let events = sqlx::query_as!(
        events::Event,
        "SELECT id, name, date, description, venue, status as \"status: events::EventStatus\" FROM events"

    ).fetch_all(&pool).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let events_with_lineup = get_events_with_lineup(&pool, events).await?;

    Ok(Json(serde_json::json!(events_with_lineup)))
}

async fn get_events_with_lineup(
    pool: &PgPool,
    events: Vec<Event>,
) -> Result<Vec<EventWithLineup>, axum::http::StatusCode> {
    let mut events_with_lineup = Vec::new();

    for event in events {
        let lineup = fetch_lineup(&pool, event.id)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        events_with_lineup.push(EventWithLineup {
            id: event.id,
            name: event.name,
            date: event.date,
            description: event.description,
            venue: event.venue,
            status: event.status,
            lineup,
        });
    }

    Ok(events_with_lineup)
}

// Plain helper function — just takes the pool and id
async fn fetch_lineup(pool: &PgPool, event_id: i32) -> Result<Vec<Artist>, sqlx::Error> {
    sqlx::query_as!(
        Artist,
        "SELECT a.id, a.name, a.instagram_url, a.spotify_url, a.photo_url
         FROM artists a
         JOIN event_artists ea ON ea.artist_id = a.id
         WHERE ea.event_id = $1",
        event_id
    )
    .fetch_all(pool)
    .await
}

#[debug_handler]
async fn get_lineup_for_event(
    State(pool): State<PgPool>,
    Path(event_id): Path<i32>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    //Return the lineup to the Json
    let lineup = sqlx::query_as!(
        Artist,
        "SELECT a.id, a.name, a.instagram_url, a.spotify_url, a.photo_url
         FROM artists a
         JOIN event_artists ea ON ea.artist_id = a.id
         WHERE ea.event_id = $1",
        event_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!(lineup)))
}

async fn get_event(
    State(pool): State<PgPool>,
    Path(event_id): Path<i32>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let event = sqlx::query_as!(
        events::Event,
        "SELECT id, name, date, description, venue, status as \"status: events::EventStatus\" FROM events WHERE id = $1",
        event_id
    ).fetch_one(&pool).await.map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let lineup = fetch_lineup(&pool, event_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let resposne = EventWithLineup {
        id: event.id,
        name: event.name,
        date: event.date,
        description: event.description,
        venue: event.venue,
        status: event.status,
        lineup,
    };

    Ok(Json(serde_json::json!(resposne)))
}

#[debug_handler]
async fn get_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, created_at FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": user.id,
        "name": user.name,
        "email": user.email
    })))
}
