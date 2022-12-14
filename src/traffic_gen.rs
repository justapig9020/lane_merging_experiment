use itertools::concat;
use rand_distr::Distribution;
use rand_distr::Poisson;

#[derive(Debug)]
struct Lane {
    earlist_arrival_times: Vec<usize>,
}

#[derive(Debug)]
pub struct Traffic {
    lanes: Vec<Lane>,
}

impl Traffic {
    pub fn new(lamdba: f32, count: usize) -> Self {
        let lane_count = 2;
        let mut lanes = Vec::with_capacity(lane_count);
        for _ in 0..lane_count {
            lanes.push(Lane::new(lamdba, count));
        }
        Self { lanes }
    }
}

impl Lane {
    fn new(lamdba: f32, count: usize) -> Self {
        let poi = Poisson::new(lamdba).unwrap();
        let mut sample = rand::thread_rng();
        let mut arrival_count = Vec::with_capacity(count);
        let mut total = 0;
        while total < count {
            let t = poi.sample(&mut sample) as usize;
            arrival_count.push(t);
            total += t;
        }
        let mut arrival_times = concat(arrival_count.iter().enumerate().map(|(t, c)| vec![t; *c]));
        arrival_times.resize(count, 0);
        Self {
            earlist_arrival_times: arrival_times,
        }
    }
}
