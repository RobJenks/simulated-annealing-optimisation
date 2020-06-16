use crate::base::*;

pub struct SolverResult<TState> {
    solver_id: SolverId,
    state: TState,
}

impl <TState> SolverResult<TState> {
    pub fn new(id: SolverId, state: TState) -> Self {
        println!("#NEW SOLVER_RESULT ({})", id);
        Self { solver_id: id, state }
    }

    pub fn get_solver_id(&self) -> SolverId { self.solver_id }
    pub fn get_state(&self) -> &TState { &self.state }
}