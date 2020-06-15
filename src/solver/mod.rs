pub mod command;
pub mod result;
pub mod signal;

use crate::system::solvable::Solvable;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::solver::command::Command;
use crate::system::state::State;
use crate::base::{SolverId, Temp};
use crate::solver::signal::SolverSignal;
use crate::solver::result::SolverResult;

pub struct Solver<TState>
    where TState: State {

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
    where TState: State {

    pub fn new(id: SolverId, data: Box<dyn Solvable<TState>>, result_output: Sender<SolverResult<TState>>) -> Self {
        let (command_inbound, command_receiver) = channel();
        let (signal_sender, signal_outbound) = channel();
        let (result_output, result_outbound) = channel();

        Self { id, data, command_inbound, command_receiver, signal_outbound, signal_sender, result_output }
    }

    pub fn start(&self) {

    }

    fn within_temp_threshold(&self, sys_temp: Temp) -> bool {
        sys_temp < self.data.get_temp_termination_threshold()
    }

    pub fn get_id(&self) -> &SolverId { &self.id }

    pub fn inbound_command(&self) -> &Sender<Command> { &self.command_inbound }
    pub fn outbound_signal(&self) -> &Receiver<SolverSignal> { &self.signal_outbound }

}
