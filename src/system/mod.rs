pub mod solvable;
pub mod state;
pub mod results;

use std::sync::mpsc::{channel, Sender, Receiver};
use crate::base::*;
use crate::solver::{Solver, command::Command};
use solvable::Solvable;
use results::Results;
use std::borrow::BorrowMut;
use crate::system::state::State;
use crate::solver::result::SolverResult;

const DEFAULT_INITIAL_POOL_SIZE: usize = 1;

pub struct System<'a, TState>
    where TState: State {

    data: &'a dyn Solvable<TState>,
    pool_target: usize,
    total_solvers_created: SolverId,

    solvers: Vec<Solver<TState>>,

    solver_tx: Sender<SolverResult<TState>>,    // mpsc sender prototype for solver instantiation
    solver_rx: Receiver<SolverResult<TState>>,  // mpsc receipt channel for solver results
}

impl<'a, TState> System<'a, TState>
    where TState: State {

    pub fn new(data: &'a dyn Solvable<TState>) -> Self {
        let (solver_tx, solver_rx) = channel();

        Self {
            data, solvers: vec![], pool_target: DEFAULT_INITIAL_POOL_SIZE, total_solvers_created: 0,
            solver_tx, solver_rx
        }
    }

    pub fn execute(&mut self) -> Results<TState> {
        let mut sys_temp = self.data.get_initial_system_temp();

        while self.within_temp_threshold(sys_temp) && !self.shutdown_requested() {
            self.handle_solver_requests();


        }
Results::new(vec![])

    }

    fn within_temp_threshold(&self, sys_temp: Temp) -> bool {
        sys_temp < self.data.get_temp_termination_threshold()
    }

    fn shutdown_requested(&self) -> bool {
        self.pool_target == 0
    }

    fn handle_solver_requests(&mut self) {
        let count = self.solvers.len();
        if self.pool_target == count { return; }

        if self.pool_target < count {
            (self.pool_target..count)
                .for_each(|_| self.terminate_solver());
        }
        else {
            let new_solvers = (count..self.pool_target)
                .map(|_| self.next_solver_id()).collect::<Vec<_>>();

            self.solvers.append(&mut new_solvers.into_iter()
                .map(|id| self.create_solver(id))
                .collect::<Vec<_>>()
            );
        }
    }

    fn create_solver(&self, id: SolverId) -> Solver<TState> {
        println!("Creating solver {}", id);

        let result_tx = self.solver_tx.clone();
        Solver::<TState>::new(id, result_tx)
    }

    fn terminate_solver(&mut self) {
        if self.solvers.is_empty() { panic!("No solver instances found during termination"); }

        self.perform_solver_termination(self.solvers.len() - 1);
    }

    fn terminate_solver_by_id(&mut self, solver_id: &SolverId) {
        let remove = self.solvers.iter()
            .position(|x| x.get_id() == solver_id)
            .unwrap_or_else(|| panic!("Missing solver instance during termination"));

        self.perform_solver_termination(remove);
    }

    fn perform_solver_termination(&mut self, ix: usize) {
        let solver = self.solvers.remove(ix);
        println!("Terminating solver '{}'", solver.get_id());

        solver
            .inbound_command()
            .send(Command::terminate())
            .unwrap_or_else(|e| panic!("Failed to issue solver shutdown command ({})", e));
    }

    fn next_solver_id(&mut self) -> SolverId {
        self.total_solvers_created += 1;
        self.total_solvers_created
    }
}