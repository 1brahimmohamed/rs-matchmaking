use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub mmr: i32,
}

#[derive(Debug, Clone)]
pub struct MatchmakingTicket {
    pub id: Uuid,
    pub player: Player,
    pub joined_at: Instant,
}

#[derive(Debug, Clone, Serialize)]
pub struct Match {
    pub match_id: Uuid,
    pub player_1: Player,
    pub player_2: Player,
}

#[derive(Debug, Serialize)]
pub struct ServerState {
    pub queue_length: usize,
    pub recent_matches: Vec<Match>,
}