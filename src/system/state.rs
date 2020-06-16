use crate::base::*;
use std::fmt::Debug;

pub trait State : Send + Sized + Clone + Sync + Debug {
    // Determine whether this state is 'better' than the given state value
    fn is_better_than(&self, other_state: &Self) -> bool;

    // Calculate the acceptance probability of this state, given a current state and system temp
    fn acceptance_probability(&self, current_state: &Self, temp: Temp) -> Prob;

    // Generate a new update from the current state
    fn generate_update(&self) -> Self;

}