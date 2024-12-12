use crate::players::{Player, Players};

pub struct FinishedGame<const M: usize, const N: usize> {
    pub winner: Option<Player>,
    pub players: Players<M, N>,
}

impl<const M: usize, const N: usize> FinishedGame<M, N> {
    pub fn new(winner: Option<Player>, players: Players<M, N>) -> Self {
        Self {
            winner,
            players,
        }
    }
}