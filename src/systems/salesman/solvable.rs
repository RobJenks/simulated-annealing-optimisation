use rand::{Rng, self};
use crate::system::state::State;
use crate::system::solvable::Solvable;
use crate::systems::salesman::state::TravellingSalesmanState;
use crate::systems::salesman::city::TravellingSalesmanCity;
use crate::base::StateCost;

const PROBLEM_AREA_DIMENSION: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct TravellingSalesmanProblem {
    cities: Vec<TravellingSalesmanCity>,
}

impl TravellingSalesmanProblem {
    pub fn new(city_count: u32) -> Self {
        Self { cities: TravellingSalesmanProblem::generate_cities(city_count) }
    }

    fn generate_cities(city_count: u32) -> Vec<TravellingSalesmanCity> {
        (0..city_count).map(|x| TravellingSalesmanProblem::generate_city(format!("City{}", x))).collect::<Vec<_>>()
    }

    fn generate_city(name: String) -> TravellingSalesmanCity {
        TravellingSalesmanCity::new(name,
                                    rand::random::<f32>() * PROBLEM_AREA_DIMENSION,
                                    rand::random::<f32>() * PROBLEM_AREA_DIMENSION)
    }

    fn calculate_total_distance(&self, state: &TravellingSalesmanState) -> f32 {
        let mut distance = 0.0;
        let route = state.get_route();
        for ix in 0..state.get_cities_in_route() - 1 {
            distance += self.cities[route[ix] as usize].distance_to(&self.cities[route[ix + 1] as usize]);
        }

        distance + self.cities[route.len() - 1].distance_to(&self.cities[0])
    }
}

impl Solvable<TravellingSalesmanState> for TravellingSalesmanProblem {
    fn generate_initial_state(&self) -> TravellingSalesmanState {
        let mut random_state = vec![];
        let mut indices = (0..self.cities.len() as u32).collect::<Vec<u32>>();
        while !indices.is_empty() {
            random_state.push(indices.remove(rand::thread_rng().gen_range(0, indices.len())));
        }

        TravellingSalesmanState::new(random_state)
    }

    fn get_initial_system_temp(&self) -> f32 {
        100.0
    }

    fn get_temp_falloff(&self) -> f32 {
        -0.001
    }

    fn get_temp_termination_threshold(&self) -> f32 {
        1.0
    }

    fn get_optimising_iteration_count(&self) -> usize {
        1
    }

    fn state_is_better_than(&self, state: &TravellingSalesmanState, other_state: &TravellingSalesmanState) -> bool {
        self.calculate_total_distance(state) < self.calculate_total_distance(other_state)
    }

    fn derive_state_cost(&self, state: &TravellingSalesmanState) -> StateCost {
        self.calculate_total_distance(state) as f64
    }

    fn state_acceptance_probability(&self, state: &TravellingSalesmanState, current_state: &TravellingSalesmanState, temp: f32) -> f32 {
        let (new_cost, current_cost) = (self.calculate_total_distance(state), self.calculate_total_distance(current_state));
        if new_cost < current_cost {
            1.0
        }
        else {
            ((current_cost - new_cost) / temp).exp()
        }
    }

    fn generate_state_update(&self, state: &TravellingSalesmanState) -> TravellingSalesmanState {
        let mut rng = rand::thread_rng();
        let n = self.cities.len() as u32;
        state.with_switched_segment(rng.gen_range(0, n), rng.gen_range(0, n))
    }

    fn clone_dyn(&self) -> Box<dyn Solvable<TravellingSalesmanState>> {
        Box::new(Self { cities: self.cities.clone() })
    }
}