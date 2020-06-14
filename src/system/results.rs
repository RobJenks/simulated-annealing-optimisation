use std::cmp::Ordering;
use crate::system::state::State;

pub struct Results<TState>
    where TState: State {

    solver_results: Vec<TState>
}

impl <TState> Results<TState>
    where TState: State {

    pub fn new(results: Vec<TState>) -> Self {
        Self { solver_results: results }
    }

    fn determine_final_result(&self) -> &TState {
        self.solver_results.iter()
            .max_by(|&x0, &x1| if x0.is_better_than(x1) { Ordering::Greater } else { Ordering::Less })
            .unwrap_or_else(|| panic!("No results!"))
    }
}