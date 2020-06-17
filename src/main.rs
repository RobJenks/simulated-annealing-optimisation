use std::sync::mpsc::channel;
use crate::systems::debug::solvable::DebugSolvable;
use crate::system::System;
use rand::Rng;
use crate::system::solvable::Solvable;
use crate::systems::salesman::solvable::TravellingSalesmanProblem;

mod base;
mod solver;
mod system;
mod systems;

fn main() {
    run_travelling_salesman();
}

fn run_debug() {
    let solvable = DebugSolvable::new();
    let mut system = System::new(solvable.clone_dyn());

    let results = system.execute();
    println!("Final result: {:?}", results.determine_final_result(&solvable));
}

fn run_travelling_salesman() {
    let solvable = TravellingSalesmanProblem::new(10);
    let mut system = System::new(solvable.clone_dyn());

    let results = system.execute();
    println!("Final result: {:?}", results.determine_final_result(&solvable));
}