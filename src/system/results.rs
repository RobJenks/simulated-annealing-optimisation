use std::cmp::Ordering;
use crate::system::state::State;
use crate::solver::result::SolverResult;
use crate::system::solvable::Solvable;

pub struct Results<TState>
    where TState: State {

    solver_results: Vec<SolverResult<TState>>
}

impl <TState> Results<TState>
    where TState: State {

    pub fn new(results: Vec<SolverResult<TState>>) -> Self {
        Self { solver_results: results }
    }

    pub fn determine_final_result(&self, solvable: &dyn Solvable<TState>) -> &TState {
        self.solver_results.iter()
            .map(|x| x.get_state())
            .max_by(|&x0, &x1| if solvable.state_is_better_than(x0, x1) { Ordering::Greater } else { Ordering::Less })
            .unwrap_or_else(|| panic!("No results!"))
    }
}