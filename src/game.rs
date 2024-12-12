use rand::Rng;

use crate::{finished_game::FinishedGame, players::{Player, Players}, state::State, weighted_state::WeightedStates};

pub struct Game<const M: usize, const N: usize> {
    current_state: State<M, N>,
    current_player: Player,
    players: Players<M, N>,
}

impl<const M: usize, const N: usize> Default for Game<M, N> {
    fn default() -> Self {
        let mut players = Players::default();
        Self {
            current_state: State::default(),
            current_player: players.next(),
            players,
        }
        
    }
}

impl<const M: usize, const N: usize> Game<M, N> {
    fn winner(&self, win_condition: usize) -> Option<Player> {
        self.current_state.winner(win_condition)
    }

    fn playable(&self) -> bool {
        self.current_state.playable()
    }

    pub fn play_until_finish(mut self, win_condition: usize, weighted_states: &mut WeightedStates<M, N>) -> FinishedGame<M, N> {
        loop {
            if let Some(player) = self.winner(win_condition) {
                return FinishedGame::new(Some(player), self.players)
            }
    
            if !self.playable() {
                return FinishedGame::new(None, self.players)
            }

            self.play(weighted_states);
        }    
    }

    fn determine_move(weights: Vec<f64>, state: &State<M, N>) -> (usize, usize) {
        let mut rand = rand::thread_rng();  
        
        let random_number: f64 = rand.gen();

        let index = weights.iter().enumerate().find_map(|(index, weight)| {
            if &random_number < weight { Some(index) } else { None }
        }).unwrap();

        let (mut m, mut n) = (index % N, index / N);

        while state.get(m, n).is_some() {
            let random_number: f64 = rand.gen();

            let index = weights.iter().enumerate().find_map(|(index, weight)| {
                if &random_number < weight { Some(index) } else { None }
            }).unwrap();

            (m, n) = (index % N, index / N);
        }

        return (m, n);
    }

    fn play(&mut self, weighted_states: &mut WeightedStates<M, N>) {
        let current_weighted_state = weighted_states.find_mut(&self.current_state).unwrap();
        let normalised_cumulative_weights = current_weighted_state.normalised_cumulative_weights();
        let (m, n) = Self::determine_move(normalised_cumulative_weights, &self.current_state);
        self.current_state.state[m][n] = Some(self.current_player.clone());
        

        let sequence = self.players.players.get_mut(&self.current_player).unwrap();
        sequence.sequence.push((self.current_state.clone(), (m, n)));

        let mut next_weighted_state = current_weighted_state.clone();
        next_weighted_state.state.state[m][n] = Some(self.current_player);
        if weighted_states.find_mut(&next_weighted_state.state).is_none() {
            weighted_states.states.push(next_weighted_state)
        }

        self.current_player = self.players.next();
    }
}