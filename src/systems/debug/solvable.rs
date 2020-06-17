use crate::system::solvable::Solvable;
use crate::systems::debug::state::DebugState;
use crate::base::{Temp, Prob, StateCost};

pub struct DebugSolvable { }

impl DebugSolvable {
    pub fn new() -> Self { Self { } }
}

impl Solvable<DebugState> for DebugSolvable {
    fn generate_initial_state(&self) -> DebugState {
        DebugState::new(0)
    }

    fn get_initial_system_temp(&self) -> f32 {
        100.0
    }

    fn get_temp_falloff(&self) -> f32 {
        -0.001
    }

    fn get_temp_termination_threshold(&self) -> f32 {
        0.0
    }

    fn get_optimising_iteration_count(&self) -> usize {
        10
    }

    fn state_is_better_than(&self, state: &DebugState, other_state: &DebugState) -> bool {
        state.get_value() > other_state.get_value()
    }

    fn derive_state_cost(&self, state: &DebugState) -> StateCost {
        state.get_value() as f64
    }

    fn state_acceptance_probability(&self, state: &DebugState, current_state: &DebugState, temp: Temp) -> Prob {
        if temp > 50.0 {
            if self.state_is_better_than(state, current_state) { 1.0 } else { 0.0 }
        }
        else {
            if self.state_is_better_than(state, current_state) { 0.0 } else { 1.0 }
        }
    }

    fn generate_state_update(&self, state: &DebugState) -> DebugState {
        DebugState::new(state.get_value() + if rand::random::<bool>() { 1 } else { -1 })  // increment [-1 +1]
    }


    fn clone_dyn(&self) -> Box<dyn Solvable<DebugState>> {
        Box::new(DebugSolvable::new())
    }
}