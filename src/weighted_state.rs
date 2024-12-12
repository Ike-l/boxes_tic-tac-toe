use rand::Rng;

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedState<const M: usize, const N: usize> {
    pub state: State<M, N>,
    pub weights: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Default for WeightedState<M, N> {
    fn default() -> Self {
        let state = State::default();
        
        let mut rand = rand::thread_rng();

        let weights = [[(); N]; M].map(|_| {
            [(); N].map(|_| rand.gen())
        });

        Self { state, weights }
    }
}

impl<const M: usize, const N: usize> WeightedState<M, N> {
    pub fn normalised_cumulative_weights(&self) -> Vec<f64> {
        let flattened = self.weights.iter().flat_map(|row| row.iter()).copied().collect::<Vec<f64>>();
        let sum = flattened.iter().sum::<f64>();

        assert!(sum > 0.0, "Sum of weights must be greater than zero.");

        let mut cumulator = 0.0;
        flattened.into_iter().map(|w| {
            cumulator += w;
            cumulator / sum
        }).collect()
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
        let mut iter = self.states.iter_mut().filter(|s| state == &s.state);
        let found = iter.next();
        assert!(iter.next().is_none(), "More than one matching state found");
        found
    }    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalised_cumulative_weights_test() {
        let weights = [[1.0, 2.0], [3.0, 4.0]];
        let expected = [0.1, 0.3, 0.6, 1.0];

        let mut weighted_state = WeightedState::<2, 2>::default();
        weighted_state.weights = weights;

        assert_eq!(weighted_state.normalised_cumulative_weights(), expected)
    }
}