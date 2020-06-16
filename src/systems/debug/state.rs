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
    fn is_better_than(&self, other_state: &Self) -> bool {
        self.value > other_state.value
    }

    fn acceptance_probability(&self, current_state: &Self, temp: f32) -> Prob {
        if temp > 50.0 {
            if self.is_better_than(current_state) { 1.0 } else { 0.0 }
        }
        else {
            if self.is_better_than(current_state) { 0.0 } else { 1.0 }
        }
    }

    fn generate_update(&self) -> Self {
        Self { value: self.value + if rand::random::<bool>() { 1 } else { -1 } }  // increment [-1 +1]
    }
}

impl Debug for DebugState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("State ({})", self.value))
    }
}