use rand::{self, Rng};
use crate::base::*;
use crate::system::state::State;
use crate::system::solvable::Solvable;
use std::sync::mpsc::channel;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct DebugState {
    value: i32,
}

impl DebugState {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> i32 { self.value }
}

impl State for DebugState {

}

impl Debug for DebugState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("State ({})", self.value))
    }
}