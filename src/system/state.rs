use crate::base::*;

pub trait State {
    // Calculate the acceptance probability of this state, given a current state and system temp
    fn acceptance_probability(&self, current_state: &Self, temp: Temp);

    // Generate a new update from the current state
    fn generate_update(&self) -> Self;

}