use std::collections::HashMap;

use crate::{finished_game::FinishedGame, players::Player};

#[derive(Debug)]
pub struct GamesStats {
    pub wins: HashMap<Player, u64>,
    pub draws: u64,
    pub rounds: u64,
}

impl GamesStats {
    pub fn new() -> Self {
        Self {
            wins: Player::to_hashmap(0),
            draws: 0,
            rounds: 1,
        }
    }

    pub fn win_rate(&self, player: &Player) -> f64 {
        *self.wins.get(player).unwrap() as f64 / self.rounds as f64
    }

    pub fn update<const M: usize, const N: usize>(&mut self, game: &FinishedGame<M, N>) {
        if let Some(player) = &game.winner {
            *self.wins.get_mut(player).unwrap() += 1;
        } else {
            self.draws += 1;
        }

        self.rounds += 1;
    }
}