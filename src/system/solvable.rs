use crate::base::*;
use crate::system::state::State;

pub trait Solvable<TState>
    where TState: State {

    // Generate an initial state for the system; can be non-deterministic and differ across calls
    fn generate_initial_state(&self) -> TState;

    fn get_initial_system_temp(&self) -> Temp;

    fn get_temp_falloff(&self) -> Temp;

    fn get_temp_termination_threshold(&self) -> Temp;

    fn get_optimising_iteration_count(&self) -> usize;

}