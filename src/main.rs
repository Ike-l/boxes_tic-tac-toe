#![deny(clippy::todo, clippy::unwrap_used)]

use std::collections::HashMap;

use small_iter_fields::{
    HashFields, IterFields
};

use rand::Rng;

#[derive(Debug, Default, IterFields, HashFields, Hash, PartialEq, Eq, Clone)]
enum Player {
    #[default]
    Crosses,
    Noughts,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct State {
    state: [[Option<Player>; N]; M]
}

impl State {
    fn win(&self, win_condition: usize) -> Option<Player> {
        todo!()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct WeightedState {
    state: State,
    weights: [[f64; N]; M],
}

impl WeightedState {
    fn normalise_weights(&self) -> [f64; N*M] {
        todo!()
    }
}

#[derive(Debug, Default)]
struct WeightedStates {
    states: Vec<WeightedState>
}

impl WeightedStates {
    fn find_mut(&mut self, state: &State) -> &mut WeightedState {
        todo!()
    }
}

#[derive(Debug, Default, Clone)]
struct Sequence {
    sequence: Vec<(State, (usize, usize))>,
}

#[derive(Debug)]
struct Players {
    players: HashMap<Player, Sequence>,
}

impl Default for Players {
    fn default() -> Self {
        let players = Player::to_hashmap(Sequence::default());

        Players { players }
    }
}

const M: usize = 3;
const N: usize = 3;
const WIN_CONDITION: usize = 3;
const WEIGHT_INCREMENT: f64 = 1.0;
const WIN_RATE: f64 = 90.0;

fn main() {
    let mut weighted_states = WeightedStates::default();
    let mut wins = Player::to_hashmap(0);
    let mut round = 1;

    let mut current_state = State::default();
    let mut players = Players::default();

    let mut player_order = Player::iter_fields().collect::<Vec<_>>().into_iter().cycle();
    let current_player = player_order.next().expect("Player enum empty");

    while (*wins.get(&Player::default()).expect("Error in `to_hashmap` in `small_iter_fields`") as f64 / round as f64) < WIN_RATE / 100.0 {
        if let Some(winner_player) = current_state.win(WIN_CONDITION) {
            for (player, sequence) in players.players {
                let increment = WEIGHT_INCREMENT * if player == winner_player { 1.0 } else { -1.0 };

                for (state, (m, n)) in &sequence.sequence {
                    let weighted_state = weighted_states.find_mut(state);
                    weighted_state.weights[*m][*n] += increment;
                }
            }
            *wins.get_mut(&winner_player).expect("Error in `to_hashmap` in `small_iter_fields`") += 1;
            round += 1;

            current_state = State::default();
            players = Players::default();
        }
        let sequence = players.players.get_mut(&current_player).expect("Error in `to_hashmap` in `small_iter_fields`");
        let current_weighted_state = weighted_states.find_mut(&current_state);
        let normalised_weights = current_weighted_state.normalise_weights();

        let mut rand = rand::thread_rng();
        let random_number: f64 = rand.gen();
        let index = normalised_weights.into_iter().rev().enumerate().find_map(|(index, weight)| {
            if weight <= random_number { Some(index) } else { None }
        }).unwrap_or(0);
        let (m, n) = (index % N, index / N);
        sequence.sequence.push((current_state.clone(), (m, n)));
        current_state.state[m][n].replace(current_player.clone());
    }

    println!("Wins: {wins:?}");
}
