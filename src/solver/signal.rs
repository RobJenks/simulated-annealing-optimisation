use crate::base::SolverId;

#[derive(Debug)]
pub enum SolverSignal {
    Noop,
    Complete(SolverId),
}