use std::collections::HashMap;
use std::time::Duration;

use itertools::Itertools;
use serde::Serialize;
use serde_json::Result;

use crate::scheduler::Schedule;
use crate::traffic_gen::Parameters;
use crate::traffic_gen::Traffic;

#[derive(Serialize)]
struct Timing(Vec<Vec<f32>>);

impl Timing {
    fn from_durations(durations: &[&[Duration]]) -> Self {
        let times = durations
            .into_iter()
            .map(|duration| duration.into_iter().map(|d| d.as_secs_f32()).collect_vec())
            .collect_vec();
        Self(times)
    }
}
#[derive(Serialize)]
pub struct Report {
    w_e: f32,
    w_p: f32,
    lambda: f32,
    n: usize,
    traffic: Timing,
    methods: HashMap<String, Timing>,
    max_delay: HashMap<String, f32>,
    mean_delay: HashMap<String, f32>,
    t_last: HashMap<String, f32>,
}

impl Report {
    pub fn new(
        para: &Parameters,
        lambda: f32,
        n: usize,
        traffic: &Traffic,
        methods: &HashMap<String, Schedule>,
    ) -> Self {
        let max_delay = methods
            .iter()
            .map(|(name, schd)| (name.clone(), traffic.max_delay_time(schd).as_secs_f32()))
            .collect();
        let mean_delay = methods
            .iter()
            .map(|(name, schd)| (name.clone(), traffic.mean_delay_time(schd).as_secs_f32()))
            .collect();
        let t_last = methods
            .iter()
            .map(|(name, schd)| (name.clone(), schd.t_last().as_secs_f32()))
            .collect();
        let traffic = Timing::from_durations(&traffic.earlist_arrival_times());
        let methods = methods
            .iter()
            .map(|(m, s)| {
                let sets = s.scheduled_entering_times();
                (m.clone(), Timing::from_durations(&sets))
            })
            .collect();
        Self {
            w_e: para.w_e.as_secs_f32(),
            w_p: para.w_p.as_secs_f32(),
            lambda,
            n,
            traffic,
            methods,
            max_delay,
            mean_delay,
            t_last,
        }
    }
}
