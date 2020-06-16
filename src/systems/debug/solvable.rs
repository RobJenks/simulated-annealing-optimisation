use crate::system::solvable::Solvable;
use crate::systems::debug::state::DebugState;

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

    fn clone_dyn(&self) -> Box<dyn Solvable<DebugState>> {
        Box::new(DebugSolvable::new())
    }
}