use crate::base::*;
use std::fmt::Debug;
use crate::system::solvable::Solvable;

pub trait State : Send + Sized + Clone + Sync + Debug {

}