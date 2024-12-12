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

enum Direction {
    Diagonal,
    Horizontal,
    Perpendicular
}

#[derive(Debug, Default, Clone, PartialEq)]
struct State {
    state: [[Option<Player>; N]; M]
}

impl State {
    fn safe_get(&self, m: usize, n: usize) -> &Option<Player> {
        self.state.get(m).and_then(|ms| ms.get(n)).unwrap_or(&None)
    }

    fn unplayable(&self) -> bool {
        let mut playable = false;

        for m in 0..M {
            for n in 0..N {
                if self.state[m][n].is_none() {
                    playable = true;
                }
            }
        }

        !playable
    }

    fn win(&self) -> Option<Player> {
        let mut dfs_diag_clone = self.clone();
        let mut dfs_horiz_clone = self.clone();
        let mut dfs_perp_clone = self.clone();
        for n in 0..N {
            for m in 0..M {
                if let Some(target_player) = self.safe_get(m, n) {
                    let diagonal_length = dfs_diag_clone.directional_backtracking(target_player, m, n, &Direction::Diagonal);
                    let horizontal_length = dfs_horiz_clone.directional_backtracking(target_player, m, n, &Direction::Horizontal);
                    let perpendocular_length = dfs_perp_clone.directional_backtracking(target_player, m, n, &Direction::Perpendicular);

                    if diagonal_length >= WIN_CONDITION || horizontal_length >= WIN_CONDITION || perpendocular_length >= WIN_CONDITION {
                        return Some(target_player.clone());
                    }
                }
            }
        }

        None
    }

    fn directional_backtracking(&mut self, target_player: &Player, m: usize, n: usize, direction: &Direction) -> usize {
        if let Some(player) = self.safe_get(m, n) {
            if player == target_player {
                self.state[m][n].take();
                let (m_p, m_n, n_p, n_n) = match direction {
                    Direction::Diagonal => {
                        (1, 1, 1, 1)
                    },
                    Direction::Horizontal => {
                        (1, 1, 0, 0)
                    },
                    Direction::Perpendicular => {
                        (0, 0, 1, 1)
                    },
                };
                let m_n = if m == 0 { 0 } else { m_n };
                let n_n = if n == 0 { 0 } else { n_n };
                1 + 
                self.directional_backtracking(target_player, m + m_p, n + n_p, direction) +
                self.directional_backtracking(target_player, m + m_p, n - n_n, direction) +
                self.directional_backtracking(target_player, m - m_n, n + n_p, direction) +
                self.directional_backtracking(target_player, m - m_n, n - n_n, direction) 
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WeightedState {
    state: State,
    weights: [[f64; N]; M],
}

impl Default for WeightedState {
    fn default() -> Self {
        let state = State::default();
        let mut weights = [[0.0; N]; M];

        let mut rand = rand::thread_rng();
        
        for m in 0..M {
            for n in 0..N {
                weights[m][n] = rand.gen()
            }
        }

        Self { state, weights }
    }
}

impl WeightedState {
    fn normalised_cumulative_weights(&self) -> [f64; N*M] {
        let flattened = self.weights.as_flattened();
        let sum: f64 = flattened.iter().sum();

        let mut normalised_weights = [0.0; N*M];
        let mut cumulator = 0.0;
        for (i, &w) in flattened.iter().enumerate() {
            normalised_weights[i] = (w + cumulator) / sum;
            cumulator += w;
        }    

        normalised_weights
    }
}

#[derive(Debug, Default)]
struct WeightedStates {
    states: Vec<WeightedState>
}

impl WeightedStates {
    fn find_mut(&mut self, state: &State) -> Option<&mut WeightedState> {
        for weighted_state in &mut self.states {
            if state == &weighted_state.state {
                return Some(weighted_state)
            }
        }
        None
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
    weighted_states.states.push(WeightedState::default());
    let mut wins = Player::to_hashmap(0);
    let mut round = 1;
    let mut passes = 0;
    let mut draws = 0;

    let mut current_state = State::default();
    let mut players = Players::default();

    let mut player_order = Player::iter_fields().collect::<Vec<_>>().into_iter().cycle();
    let mut current_player = player_order.next().expect("Player enum empty");

    while (*wins.get(&Player::default()).expect("Error in `to_hashmap` in `small_iter_fields`") as f64 / round as f64) < WIN_RATE / 100.0 && round < 50 {
        println!("Round: {round:?}");
        //println!("Current State: {current_state:?}");
        //assert!(passes <= 3);
        if let Some(winner_player) = current_state.win() {
            println!("Winner: {winner_player:?}");
            for (player, sequence) in players.players {
                let increment = WEIGHT_INCREMENT * if player == winner_player { 1.0 } else { -1.0 };

                for (state, (m, n)) in &sequence.sequence {
                    let weighted_state = weighted_states.find_mut(state).expect("State not found in weighted states");
                    weighted_state.weights[*m][*n] += increment;
                }
            }
            *wins.get_mut(&winner_player).expect("Error in `to_hashmap` in `small_iter_fields`") += 1;
            round += 1;

            current_state = State::default();
            players = Players::default();
        }
        if current_state.unplayable() {
            draws += 1;
            round += 1;
            current_state = State::default();
            players = Players::default();
        }
        let sequence = players.players.get_mut(&current_player).expect("Error in `to_hashmap` in `small_iter_fields`");
        let current_weighted_state = weighted_states.find_mut(&current_state).expect("State not found in weighted states");
        let normalised_weights = current_weighted_state.normalised_cumulative_weights();

        let mut rand = rand::thread_rng();
        let mut random_number: f64 = rand.gen();
        //println!("random_number: {random_number:?}");
        
        let mut index = normalised_weights.into_iter().rev().enumerate().find_map(|(index, weight)| {
            //println!("weight: {weight:?}");
            if weight <= random_number { Some(N*M-1-index) } else { None }
        }).unwrap_or(0);
        let (mut m, mut n) = (index % N, index / N);

        while current_state.state[m][n].is_some() {
            random_number = rand.gen();
            //println!("random_number: {random_number:?}");
            index = normalised_weights.into_iter().enumerate().find_map(|(index, weight)| {
                //println!("weight: {weight:?}");
                if weight > random_number { Some(index) } else { None }
            }).unwrap_or(0);
            //println!("index: {index:?}");
            (m, n) = (index % N, index / N);
            //println!("Current state at m,n: {:?}", current_state.state[m][n]);
            //println!("Current state: {:?}", current_state.state);
        }
        current_state.state[m][n].replace(current_player.clone());

        sequence.sequence.push((current_state.clone(), (m, n)));
        
        let mut next_weighted_state = current_weighted_state.clone();
        next_weighted_state.state.state[m][n].replace(current_player.clone());
        if weighted_states.find_mut(&next_weighted_state.state).is_none() {
            weighted_states.states.push(next_weighted_state)
        }

        current_player = player_order.next().unwrap();
        passes += 1;
    }

    println!("Wins: {wins:?}");
    println!("Draws: {draws:?}");
}

/*
Test:
-Win
-Normalise
*/