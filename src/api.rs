use axum::{Extension, Json};
use std::time::Instant;
use uuid::Uuid;
use crate::models::{Player, MatchmakingTicket, ServerState};
use crate::engine::SharedState;

pub async fn join_queue(
    Extension(state): Extension<SharedState>,
    Json(player): Json<Player>,
) -> &'static str {
    let mut matchmaker = state.lock().await;
    matchmaker.queue.push(MatchmakingTicket {
        id: Uuid::new_v4(),
        player,
        joined_at: Instant::now(),
    });
    "Joined queue successfully"
}

// endpoint for dashboard
pub async fn get_state(
    Extension(state): Extension<SharedState>
) -> Json<ServerState> {
    let matchmaker = state.lock().await;
    Json(ServerState {
        queue_length: matchmaker.queue.len(),
        recent_matches: matchmaker.recent_matches.clone(),
    })
}