use crate::base::SolverId;

pub enum Command {
    Noop,
    Terminate(SolverId),
}
