pub mod command;

use crate::system::solvable::Solvable;
use std::sync::mpsc::{Sender, Receiver};
use crate::solver::command::Command;
use crate::system::state::State;

pub struct Solver<TState>
    where TState: State {

    id: String,
    command_receiver: Receiver<Command>,
    result_channel: Sender<TState>
}

impl <TState> Solver<TState>
    where TState: State {

    pub fn new(id: String, command_receiver: Receiver<Command>, result_channel: Sender<TState>) -> Self {
        Self { id, command_receiver, result_channel }
    }

    pub fn get_id(&self) -> &String { &self.id }
}
