use crate::players::Player;

enum Direction {
    DiagonalRight,
    DiagonalLeft,
    Horizontal,
    Perpendicular
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Player(Player),
}

impl Cell {
    pub fn unwrap(self) -> Player {
        match self {
            Cell::Player(p) => p
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct State<const M: usize, const N: usize> {
    pub state: [[Option<Cell>; N]; M]
}

impl<const M: usize, const N: usize> Default for State<M, N> {
    fn default() -> Self {
        Self {
            state: [[None; N]; M]
        }
    }
}

impl<const M: usize, const N: usize> State<M, N> {
    #[allow(dead_code)]
    pub fn new(state: [[Option<Cell>; N]; M]) -> Self {
        Self {
            state
        }
    }

    pub fn get(&self, m: usize, n: usize) -> &Option<Cell> {
        self.state.get(m).and_then(|ms| ms.get(n)).unwrap_or(&None)
    }

    pub fn playable(&self) -> bool {
        self.state.iter().flatten().any(|cell| cell.is_none())
    }

    pub fn winner(&self, win_condition: usize) -> Option<Player> {
        let mut dfs_diag_right_clone = self.clone();
        let mut dfs_diag_left_clone = self.clone();
        let mut dfs_horiz_clone = self.clone();
        let mut dfs_perp_clone = self.clone();
        for n in 0..N {
            for m in 0..M {
                if let Some(target_player) = self.get(m, n) {
                    let target_player = &target_player.unwrap();
                    let diagonal_right_length = dfs_diag_right_clone.directional_backtracking(target_player, m, n, &Direction::DiagonalRight);
                    let diagonal_left_length = dfs_diag_left_clone.directional_backtracking(target_player, m, n, &Direction::DiagonalLeft);
                    let horizontal_length = dfs_horiz_clone.directional_backtracking(target_player, m, n, &Direction::Horizontal);
                    let perpendocular_length = dfs_perp_clone.directional_backtracking(target_player, m, n, &Direction::Perpendicular);
                    if diagonal_right_length >= win_condition || diagonal_left_length >= win_condition || horizontal_length >= win_condition || perpendocular_length >= win_condition {
                        return Some(target_player.clone());
                    }
                }
            }
        }

        None
    }

    fn directional_backtracking(&mut self, target_player: &Player, m: usize, n: usize, direction: &Direction) -> usize {
        if let Some(player) = self.get(m, n) {
            let player = &player.unwrap();
            if player == target_player {
                self.state[m][n].take();
                1 + match direction {
                    Direction::DiagonalRight => {
                        let mut sum = 0;
                        if m != 0 && n != 0 {
                            sum += self.directional_backtracking(target_player, m - 1, n - 1, direction);
                        }
                        sum += self.directional_backtracking(target_player, m + 1, n + 1, direction);
                        
                        sum
                    },
                    Direction::DiagonalLeft => {
                        let mut sum = 0;
                        if m != 0 {
                            sum += self.directional_backtracking(target_player, m - 1, n + 1, direction);
                        }
                        if n != 0 {
                            sum += self.directional_backtracking(target_player, m + 1, n - 1, direction);
                        }

                        sum
                    }
                    Direction::Horizontal => {
                        let mut sum = 0;
                        if m != 0 {
                            sum += self.directional_backtracking(target_player, m - 1, n, direction);
                        }
                        sum += self.directional_backtracking(target_player, m + 1, n, direction);
                        sum
                    },
                    Direction::Perpendicular => {
                        let mut sum = 0;
                        if n != 0 {
                            sum += self.directional_backtracking(target_player, m, n - 1, direction);
                        }
                        sum += self.directional_backtracking(target_player, m, n + 1, direction);
                        sum
                    },
                }                
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const M: usize = 3;
    const N: usize = 3;
    const WIN_CONDITION: usize = 3;

    fn winning_states() -> Vec<State<M, N>> {
        let c = Cell::Player(Player::Crosses);
        let n = Cell::Player(Player::Noughts);
        vec![
            State::new([
                [Some(c.clone()), None, None], 
                [Some(n.clone()), Some(c.clone()), None], 
                [None, Some(n.clone()), Some(c.clone())]
            ]),
            State::new([
                [Some(c.clone()), Some(n.clone()), None], 
                [Some(c.clone()), None, Some(n.clone())], 
                [Some(c.clone()), None, None]
            ]),
            State::new([
                [Some(c.clone()), None, Some(c.clone())], 
                [Some(c.clone()), None, None], 
                [Some(n.clone()), Some(n.clone()), Some(n.clone())]
            ]),
        ]
    }

    fn playable_states() -> Vec<State<M, N>> {
        let c = Cell::Player(Player::Crosses);
        let n = Cell::Player(Player::Noughts);
        vec![
            State::new([
                [Some(c.clone()), None, None], 
                [Some(n.clone()), None, None], 
                [None, Some(n.clone()), Some(c.clone())]
            ]),
            State::new([
                [Some(c.clone()), Some(n.clone()), None], 
                [None, None, Some(n.clone())], 
                [Some(c.clone()), None, None]
            ]),
            State::new([
                [Some(c.clone()), None, Some(c.clone())], 
                [Some(c.clone()), None, None], 
                [Some(n.clone()), None, Some(n.clone())]
            ]),
            State::new([
                [Some(n.clone()), None, None],
                [Some(n.clone()), Some(c.clone()), None],
                [Some(c.clone()), Some(n.clone()), None]
            ])
        ]  
    }

    fn unplayable_states() -> Vec<State<M, N>> {
        let c = Cell::Player(Player::Crosses);
        let n = Cell::Player(Player::Noughts);
        vec![
            State::new([[Some(c.clone()); N]; M]),
            State::new([[Some(n.clone()); N]; M])
        ]  
    }

    #[test]
    fn winner_test() {
        assert!(winning_states().iter().all(|s| s.winner(WIN_CONDITION).is_some()));
        assert!(playable_states().iter().all(|s| s.winner(WIN_CONDITION).is_none()));
    }

    #[test]
    fn playable_test() {
        assert!(playable_states().iter().all(|s| s.playable()));
        assert!(unplayable_states().iter().all(|s| !s.playable()));
    }
}