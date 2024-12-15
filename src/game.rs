use rand::Rng;

use crate::{finished_game::FinishedGame, players::{Player, Players}, state::{Cell, State}, weighted_state::{WeightedCell, WeightedState, WeightedStates}};

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

            self.play(weighted_states, false);
        }    
    }

    pub fn play_versus_ai(mut self, win_condition: usize, weighted_states: &mut WeightedStates<M, N>) {
        loop {
            if let Some(player) = self.winner(win_condition) {
                println!("Winner: {player:?}");
                println!("Final state: {:?}", self.current_state);
                return
            }
    
            if !self.playable() {
                return
            }

            println!("State: {:?}", self.current_state);
            
            println!("Weights: {:?}", weighted_states.find_mut(&self.current_state));
            
            let mut input = String::new();

            println!("Enter like `m n`:");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            let inputs: Vec<usize> = input.trim().split_whitespace().map(|num| num.parse().expect("Please enter valid numbers")).collect();
            let (m, n) = (inputs[0], inputs[1]);

            let was = self.play_move(m, n);
            println!("Was: {was:?}, Now: {:?}", self.current_player);

            self.current_player = self.players.next();

            self.play(weighted_states, true);
        }  
    }

    fn random_move(state: &WeightedState<M, N>) -> (usize, usize) {
        let (mut m, mut n) = (0, 0);
        while state.just_state().get(m, n).is_some() {
            m = rand::thread_rng().gen_range(0..M);
            n = rand::thread_rng().gen_range(0..N);   
        }

        (m, n)
    }

    fn determine_move(state: &WeightedState<M, N>) -> (usize, usize) {
        let mut index = (0, 0);
        let mut max = core::f64::MIN;

        for m in 0..M {
            for n in 0..N {
                if let WeightedCell::Weight(w) = state.state[m][n] {
                    if w > max {
                        max = w;

                        index = (m, n)
                    }
                }
            }
        }
 
        return index;
    }

    pub fn play_move(&mut self, m: usize, n: usize) -> Option<Cell> {
        self.current_state.state[m][n].replace(Cell::Player(self.current_player))
    }

    fn play(&mut self, weighted_states: &mut WeightedStates<M, N>, debug_flag: bool) {
        let current_weighted_state = match weighted_states.find_mut(&self.current_state) {
            Some(s) => s,
            // would mean the bot has never encountered this position
            // (can be avoided by storing every position instead of building it up as i go)
            //None => panic!("Weighted States: {weighted_states:?}\nLength: {:?}", weighted_states.states.len())
            None => panic!("Length: {:?}", weighted_states.states.len())
        };

        let (m, n) = if rand::random::<f64>() < 0.9 {
            Self::random_move(&current_weighted_state)
        } else {
            Self::determine_move(&current_weighted_state)
        };

        if debug_flag {
            println!("Weights: {:?}", current_weighted_state.state);
            println!("AI played: {m} {n}");
        }

        let sequence = self.players.players.get_mut(&self.current_player).unwrap();
        sequence.sequence.push((self.current_state.clone(), (m, n)));

        self.play_move(m, n);
        

        let mut next_weighted_state: WeightedState<M, N> = current_weighted_state.clone();
        next_weighted_state.state[m][n] = WeightedCell::Cell(Cell::Player(self.current_player));
        if weighted_states.find_mut(&next_weighted_state.just_state()).is_none() {
            next_weighted_state.clear_weights();
            weighted_states.states.push(next_weighted_state)
        }

        self.current_player = self.players.next();
    }
}