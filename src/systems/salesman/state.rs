use crate::system::state::State;

#[derive(Debug, Clone)]
pub struct TravellingSalesmanState {
    route: Vec<u32>,
}

impl TravellingSalesmanState {
    pub fn new(route: Vec<u32>) -> Self {
        Self { route }
    }

    pub fn with_switched_segment(&self, x0: u32, x1: u32) -> Self {
        let mut new_data = self.route.clone();
        let val0 = self.route[x0 as usize];
        new_data[x0 as usize] = new_data[x1 as usize];
        new_data[x1 as usize] = val0;

        Self { route: new_data }
    }

    pub fn get_route(&self) -> &Vec<u32> { &self.route }
    pub fn get_cities_in_route(&self) -> usize { self.route.len() }
}

impl State for TravellingSalesmanState {

}