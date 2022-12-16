use std::time::Duration;

use itertools::concat;
use itertools::Itertools;
use rand_distr::Distribution;
use rand_distr::Poisson;
use serde::Serialize;

use crate::scheduler::Schedule;

#[derive(Debug)]
pub struct Lane(Vec<Duration>);

#[derive(Debug)]
pub struct Traffic {
    lanes: Vec<Lane>,
}

impl Traffic {
    pub fn generate(lambda: f32, count: usize) -> Self {
        let lane_count = 2;
        let mut lanes = Vec::with_capacity(lane_count);
        for _ in 0..lane_count {
            lanes.push(Lane::generate(lambda, count));
        }
        Self { lanes }
    }
    pub fn earlist_arrival_times(&self) -> Vec<&[Duration]> {
        self.lanes
            .iter()
            .map(|lane| lane.0.as_slice())
            .collect_vec()
    }
    pub fn delay_times(&self, schd: &Schedule) -> Vec<Vec<Duration>> {
        let sets = schd.scheduled_entering_times();
        self.lanes
            .iter()
            .zip(sets.iter())
            .map(|(eat, set)| {
                eat.times()
                    .iter()
                    .zip(set.iter())
                    .map(|(e, s)| *s - *e)
                    .collect_vec()
            })
            .collect_vec()
    }
    pub fn mean_delay_time(&self, schd: &Schedule) -> Duration {
        let dts = self.delay_times(&schd);
        dts.iter()
            .map(|dt| dt.iter().sum::<Duration>() / dt.len() as u32)
            .sum::<Duration>()
            / dts.len() as u32
    }
    pub fn max_delay_time(&self, schd: &Schedule) -> Duration {
        let dts = self.delay_times(&schd);
        dts.iter()
            .map(|dt| {
                dt.iter()
                    .max()
                    .and_then(|m| Some(m.clone()))
                    .unwrap_or_default()
            })
            .max()
            .and_then(|m| Some(m.clone()))
            .unwrap_or_default()
    }
}

impl Lane {
    pub fn new(times: Vec<Duration>) -> Self {
        Lane(times)
    }
    fn generate(lambda: f32, count: usize) -> Self {
        let poi = Poisson::new(lambda).unwrap();
        let mut sample = rand::thread_rng();
        let mut arrival_count = Vec::with_capacity(count);
        let mut total = 0;
        while total < count {
            let t = poi.sample(&mut sample) as usize;
            arrival_count.push(t);
            total += t;
        }
        let mut arrival_times = concat(
            arrival_count
                .iter()
                .enumerate()
                .map(|(t, c)| vec![Duration::from_secs(t as u64); *c]),
        );
        arrival_times.resize(count, Duration::default());
        Self(arrival_times)
    }
    pub fn times(&self) -> &[Duration] {
        &self.0
    }
}

pub struct Parameters {
    pub w_p: Duration,
    pub w_e: Duration,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            w_p: Duration::from_secs(3),
            w_e: Duration::from_secs(1),
        }
    }
}
