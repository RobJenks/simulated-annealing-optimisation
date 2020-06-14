pub mod solvable;
pub mod state;

use solvable::Solvable;

pub struct System<'a, TState> {
    data: &'a dyn Solvable<TState>
}