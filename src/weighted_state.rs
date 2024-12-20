use crate::state::{Cell, State};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeightedCell {
    Cell(Cell),
    Weight(isize)
}

impl Default for WeightedCell {
    fn default() -> Self {
        Self::Weight(1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedState<const M: usize, const N: usize> {
    pub state: [[WeightedCell; N]; M],
}

impl<const M: usize, const N: usize> Default for WeightedState<M, N> {
    fn default() -> Self {
        let state = [[WeightedCell::default(); N]; M];
        Self { state }
    }
}

impl<const M: usize, const N: usize> WeightedState<M, N> {
    pub fn clear_weights(&mut self) {
        for m in 0..M {
            for n in 0..N {
                if let WeightedCell::Weight(_) = self.state[m][n] {
                    self.state[m][n] = WeightedCell::default()
                }
            }
        }
    }

    pub fn just_state(&self) -> State<M, N> {
        let mut state = State::default();
        for m in 0..M {
            for n in 0..N {
                if let WeightedCell::Cell(c) = self.state[m][n] {
                    state.state[m][n] = Some(c);
                }
            }
        }
        state
    }
}

#[derive(Debug)]
pub struct WeightedStates<const M: usize, const N: usize> {
    pub states: Vec<WeightedState<M, N>>
}

impl<const M: usize, const N: usize> Default for WeightedStates<M, N> {
    fn default() -> Self {
        Self {
            states: vec![WeightedState::default()]
        }
    }
}


impl<const M: usize, const N: usize> WeightedStates<M, N> {
    pub fn find_mut(&mut self, state: &State<M, N>) -> Option<&mut WeightedState<M, N>> {
        let mut iter = self.states.iter_mut().filter(|s| state == &s.just_state());
        let found = iter.next();
        assert!(iter.next().is_none(), "More than one matching state found");
        found
    }    
}
