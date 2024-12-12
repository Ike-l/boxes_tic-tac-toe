use std::{collections::HashMap, iter::Cycle, vec::IntoIter};

use small_iter_fields::{
    HashFields, IterFields
};

use crate::state::State;

#[derive(Debug, Default, IterFields, HashFields, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    #[default]
    Crosses,
    Noughts,
}

#[derive(Debug, Default, Clone)]
pub struct Sequence<const M: usize, const N: usize> {
    pub sequence: Vec<(State<M, N>, (usize, usize))>,
}

#[derive(Debug)]
pub struct Players<const M: usize, const N: usize> {
    pub players: HashMap<Player, Sequence<M, N>>,
    player_order: Cycle<IntoIter<Player>>
}

impl<const M: usize, const N: usize> Default for Players<M, N> {
    fn default() -> Self {
        let players = Player::to_hashmap(Sequence::default());
        let player_order = Player::iter_fields().collect::<Vec<_>>().into_iter().cycle();

        Players { players, player_order }
    }
}

impl<const M: usize, const N: usize> Players<M, N> {
    pub fn next(&mut self) -> Player {
        self.player_order.next().expect("Player `enum` is empty")
    }
}