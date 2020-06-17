use crate::base::*;
use crate::system::state::State;

pub trait Solvable<TState> : Send
    where TState: State {

    // Generate an initial state for the system; can be non-deterministic and differ across calls
    fn generate_initial_state(&self) -> TState;

    fn get_initial_system_temp(&self) -> Temp;

    fn get_temp_falloff(&self) -> Temp;

    fn get_temp_termination_threshold(&self) -> Temp;

    fn get_optimising_iteration_count(&self) -> usize;

    // Determine whether a state is 'better' than the other given state value
    fn state_is_better_than(&self, state: &TState, other_state: &TState) -> bool;

    // Return the aggregate cost of a given state in numeric form; mapping is implementation-dependent
    fn derive_state_cost(&self, state: &TState) -> StateCost;

    // Calculate the acceptance probability of this state, given a current state and system temp
    fn state_acceptance_probability(&self, state: &TState, current_state: &TState, temp: Temp) -> Prob;

    // Generate a new update from the given state
    fn generate_state_update(&self, state: &TState) -> TState;


    // Must expose a dynamic clone
    fn clone_dyn(&self) -> Box<dyn Solvable<TState>>;

}