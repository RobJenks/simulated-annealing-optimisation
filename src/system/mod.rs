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
use std::time::Duration;
use crate::solver::signal::SolverSignal;

const DEFAULT_INITIAL_POOL_SIZE: usize = 1;
const DEFAULT_RESULT_CHANNEL_TIMEOUT_SECS: u64 = 3;

pub struct System<TState>
    where TState: 'static + State {

    data: Box<dyn Solvable<TState>>,
    pool_target: usize,
    total_solvers_created: SolverId,

    solvers: Vec<Solver<TState>>,

    solver_tx: Sender<SolverResult<TState>>,    // mpsc sender prototype for solver instantiation
    solver_rx: Receiver<SolverResult<TState>>,  // mpsc receipt channel for solver results
}

impl<TState> System<TState>
    where TState: 'static + State {

    pub fn new(data: Box<dyn Solvable<TState>>) -> Self {
        let (solver_tx, solver_rx) = channel();

        Self {
            data: data.clone_dyn(), solvers: vec![],
            pool_target: DEFAULT_INITIAL_POOL_SIZE, total_solvers_created: 0,
            solver_tx, solver_rx
        }
    }

    pub fn execute(&mut self) -> Results<TState> {
        let mut results = vec![];

        while self.has_active_solvers() || self.requires_active_solvers() {
            self.handle_solver_requests();
            self.handle_solver_signals();

            self.solver_rx.recv_timeout(Duration::from_secs(DEFAULT_RESULT_CHANNEL_TIMEOUT_SECS))
                .and_then(|x| Ok(results.push(x)))
                .unwrap_or_else(|_| ());
        }

        Results::new(results)
    }

    fn requires_active_solvers(&self) -> bool { self.pool_target != 0 }
    fn shutdown_requested(&self) -> bool {
        !self.requires_active_solvers()
    }

    fn has_active_solvers(&self) -> bool { !self.solvers.is_empty() }

    fn handle_solver_signals(&mut self) {
        let signals = self.solvers.iter()
            .map(|x| x.outbound_signal().try_recv())
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();

        signals.iter().for_each(|x| self.handle_solver_signal(x));
    }

    fn handle_solver_signal(&mut self, signal: &SolverSignal) {
        println!("Received solver signal: {:?}", signal);
        match signal {
            SolverSignal::Complete(id) => self.handle_solver_completion(*id),
            _ => println!("Unrecognised signal!"),
        }
    }

    fn handle_solver_completion(&mut self, id: SolverId) {
        let ix = self.solvers.iter()
            .position(|x| x.get_id() == &id)
            .unwrap_or_else(|| panic!("Received completion signal for unknown solver"));

        println!("Solver {} deregistered by system", self.solvers.get(ix).unwrap().get_id());
        self.solvers.remove(ix);

        self.pool_target -= 1;  // Reduce target pool size on successful completion, to prevent immediate re-instantiation
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
        let solver = Solver::<TState>::new(id, self.data.clone_dyn(), result_tx);

        solver.start();
        solver
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
        let solver = self.solvers.get(ix)
            .unwrap_or_else(|| panic!("Solver termination requested for invalid ix"));

        println!("Terminating solver '{}'", solver.get_id());
        solver
            .inbound_command()
            .send(Command::Terminate(*solver.get_id()))
            .unwrap_or_else(|e| panic!("Failed to issue solver shutdown command ({})", e));
    }

    fn next_solver_id(&mut self) -> SolverId {
        self.total_solvers_created += 1;
        self.total_solvers_created
    }
}