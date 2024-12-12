mod players;
mod state;
mod weighted_state;
mod game;
mod games_stats;
mod finished_game;

use games_stats::GamesStats;
use players::Player;

use game::Game;
use weighted_state::WeightedStates;

fn main() {
    let win_condition = 3;
    let expected_win_rate = 90.0;

    let mut weighted_states = WeightedStates::<3, 3>::default();
    let mut games_stats = GamesStats::new();

    while games_stats.win_rate(&Player::default()) < expected_win_rate / 100.0 && games_stats.rounds < 1000000 {
        let game = Game::default();
        let finished_game = game.play_until_finish(win_condition, &mut weighted_states);
        games_stats.update(&finished_game);

        println!("Round: {:?}", games_stats.rounds);

        if let Some(winner) = finished_game.winner {
            for (player, sequence) in finished_game.players.players {
                let increment = if player == winner { 1.01 } else { 0.99 };
                for (state, (m, n)) in &sequence.sequence {
                    let weighted_state = weighted_states.find_mut(state).unwrap();
                    let weight = &mut weighted_state.weights[*m][*n];
                    *weight *= increment;
                    *weight = weight.clamp(0.1, 1000.0);
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
