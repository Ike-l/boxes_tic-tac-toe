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
    let weight_increment = 1.0;
    let expected_win_rate = 90.0;

    let mut weighted_states = WeightedStates::<3, 3>::default();
    let mut games_stats = GamesStats::new();

    while games_stats.win_rate(&Player::default()) < expected_win_rate / 100.0 && games_stats.rounds < 50 {
        let game = Game::default();
        let finished_game = game.play_until_finish(win_condition, &mut weighted_states);
        games_stats.update(&finished_game);
        println!("Game finished, updated stats: {games_stats:?}");

        if let Some(winner) = finished_game.winner {
            for (player, sequence) in finished_game.players.players {
                let increment = weight_increment * if player == winner { 1.0 } else { -1.0 };
                for (state, (m, n)) in &sequence.sequence {
                    let weighted_state = weighted_states.find_mut(state).unwrap();
                    weighted_state.weights[*m][*n] += increment;
                }
            }
        }
    }

    println!("Games Stats: {games_stats:?}");
}
