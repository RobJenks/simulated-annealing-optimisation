pub mod command;
pub mod result;
pub mod signal;

use rand::{self, Rng};
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::system::solvable::Solvable;
use crate::solver::command::Command;
use crate::system::state::State;
use crate::base::{SolverId, Temp, Prob};
use crate::solver::signal::SolverSignal;
use crate::solver::result::SolverResult;
use std::time::Duration;
use rand::rngs::ThreadRng;

pub struct Solver<TState>
    where TState: 'static + State {

    id: SolverId,
    data: Box<dyn Solvable<TState>>,

    // Channel interfaces exposed to caller
    command_inbound: Sender<Command>,
    signal_outbound: Receiver<SolverSignal>,

    // Channel interface for async worker processes
    command_receiver: Receiver<Command>,
    signal_sender: Sender<SolverSignal>,
    result_output: Sender<SolverResult<TState>>,        // Provided by caller


}

impl <TState> Solver<TState>
    where TState: 'static + State {

    pub fn new(id: SolverId, data: Box<dyn Solvable<TState>>, result_output: Sender<SolverResult<TState>>) -> Self {
        let (command_inbound, command_receiver) = channel();
        let (signal_sender, signal_outbound) = channel();

        Self { id, data, command_inbound, command_receiver, signal_outbound, signal_sender, result_output }
    }

    pub fn start(&self) {
        let id = self.id;
        let mut sys_temp = self.data.get_initial_system_temp();
        let sys_temp_falloff = self.data.get_temp_falloff();
        let sys_temp_threshold = self.data.get_temp_termination_threshold();

        let solvable = self.data.clone_dyn();
        let initial_state = self.data.generate_initial_state();
        let output = self.result_output.clone();

        let thread = std::thread::spawn(move || {
            println!("#STARTING SOLVER");
            let mut rng = rand::thread_rng();
            let solv = solvable.clone_dyn();
            let mut state = initial_state;

            while Solver::<TState>::within_temp_threshold(sys_temp, sys_temp_threshold) {
                let candidate = solv.generate_state_update(&state);
                if Solver::<TState>::state_accepted(&candidate, &state, sys_temp, &solv, &mut rng) {
                    print!("{:.2}, ", solv.derive_state_cost(&state));
                    state = candidate;
                }

                sys_temp += sys_temp_falloff;
            }

            output.send(SolverResult::new(id, state));
        });

        thread.join().unwrap_or_else(|_| panic!("Failed to join temporary solver thread"));
        self.signal_sender.send(SolverSignal::Complete(self.id));
        println!("#SOLVER DONE");
    }

    fn state_accepted(candidate: &TState, current: &TState, sys_temp: Temp,
                      solvable: &Box<dyn Solvable<TState>>, rng: &mut ThreadRng) -> bool {
        solvable.state_acceptance_probability(candidate, current, sys_temp) > rng.gen::<Prob>()
    }

    fn within_temp_threshold(sys_temp: Temp, sys_temp_threshold: Temp) -> bool {
        sys_temp > sys_temp_threshold
    }

    pub fn get_id(&self) -> &SolverId { &self.id }

    pub fn inbound_command(&self) -> &Sender<Command> { &self.command_inbound }
    pub fn outbound_signal(&self) -> &Receiver<SolverSignal> { &self.signal_outbound }

}
