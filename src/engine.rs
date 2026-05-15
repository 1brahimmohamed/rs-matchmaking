use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::models::{Match, MatchmakingTicket};

pub type SharedState = Arc<Mutex<Matchmaker>>;

pub struct Matchmaker {
    pub queue: Vec<MatchmakingTicket>,
    pub recent_matches: Vec<Match>,
}

impl Matchmaker {
    pub fn new() -> Self {
        Self { 
            queue: Vec::new(),
            recent_matches: Vec::new(),
        }
    }
}

pub async fn run_matchmaking_loop(state: SharedState) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    loop {
        interval.tick().await;
        let mut matchmaker = state.lock().await;
        
        if matchmaker.queue.len() < 2 {
            continue; 
        }

        matchmaker.queue.sort_by_key(|ticket| ticket.player.mmr);

        let mut matches_formed = Vec::new();
        let mut i = 0;

        while i + 1 < matchmaker.queue.len() {
            let p1 = &matchmaker.queue[i];
            let p2 = &matchmaker.queue[i + 1];

            if (p1.player.mmr - p2.player.mmr).abs() <= 100 {
                let ticket_1 = matchmaker.queue.remove(i);
                let ticket_2 = matchmaker.queue.remove(i); 

                matches_formed.push(Match {
                    match_id: Uuid::new_v4(),
                    player_1: ticket_1.player,
                    player_2: ticket_2.player,
                });
            } else {
                i += 1; 
            }
        }

        // Save matches to state and keep only the last 10 so memory doesn't leak
        for m in matches_formed {
            println!("Match: {} vs {}", m.player_1.id, m.player_2.id);
            matchmaker.recent_matches.push(m);
        }
        
        if matchmaker.recent_matches.len() > 10 {
            let excess = matchmaker.recent_matches.len() - 10;
            matchmaker.recent_matches.drain(0..excess);
        }
    }
}