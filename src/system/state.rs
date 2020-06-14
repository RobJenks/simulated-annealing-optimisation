use crate::base::*;

pub trait State<TVal> {
    fn acceptance_probability(&self, current_state: &TVal, temp: Temp) {

    }
}