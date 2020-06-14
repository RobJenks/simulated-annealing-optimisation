use crate::base::*;

pub trait Solvable<TState> {
    // Generate an initial state for the system; can be non-deterministic and differ across calls
    fn generate_initial_state(&self) -> TState;

    fn initial_system_temp(&self) -> Temp;

}