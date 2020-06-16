use std::sync::mpsc::channel;
use crate::systems::debug::solvable::DebugSolvable;
use crate::system::System;
use rand::Rng;

mod base;
mod solver;
mod system;
mod systems;

fn main() {
    let solvable = DebugSolvable::new();
    let mut system = System::new(Box::new(solvable));

    let results = system.execute();
    println!("Final result: {:?}", results.determine_final_result());
}

