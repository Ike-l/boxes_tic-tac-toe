mod players;
mod state;
mod weighted_state;
mod game;
mod games_stats;
mod finished_game;

use finished_game::FinishedGame;
use games_stats::GamesStats;

use game::Game;
use weighted_state::{WeightedCell, WeightedStates};

fn main() {
    // 3 in a row
    let win_condition = 3;

    // 3x3 grid
    let mut weighted_states: WeightedStates<3, 3> = WeightedStates::default();

    let mut games_stats = GamesStats::default();

    while games_stats.rounds <= 500_000 {
        if games_stats.rounds % 10000 == 0 {
            println!("Round: {:?}", games_stats.rounds);
        }

        let game: Game<3, 3> = Game::default();
        let finished_game: FinishedGame<3, 3> = game.play_until_finish(win_condition, &mut weighted_states);
        games_stats.update(&finished_game);

        for (player, sequence) in finished_game.players.players {
            // reward winner by increasing weight of their moves by 1%, punish losing OR drawing by subtracting 1%
            let increment = if finished_game.winner.is_some() && finished_game.winner.unwrap() == player {
                1
            } else if finished_game.winner.is_none() {
                0
            } else {
                -1
            };
    
            for (state, (m, n)) in &sequence.sequence {
                let weighted_state = weighted_states.find_mut(state).unwrap();
                match &mut weighted_state.state[*m][*n] {
                    // somehow the index of the `maximum weight`
                    WeightedCell::Cell(_) => panic!("Found cell!"),
                    WeightedCell::Weight(w) => *w += increment,
                }
            }
        }

    }

    println!("Games Stats: {games_stats:?}");

    loop {
        let game = Game::default();
        game.play_versus_ai(win_condition, &mut weighted_states);
    }
}
