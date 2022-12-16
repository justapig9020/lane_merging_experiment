use std::collections::HashMap;
use std::time::Duration;

use itertools::Itertools;
use serde::Serialize;
use serde_json::Result;

use crate::scheduler::Schedule;
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
    traffic: Timing,
    methods: HashMap<String, Timing>,
}

impl Report {
    pub fn new(traffic: &Traffic, methods: &HashMap<String, Schedule>) -> Self {
        let traffic = Timing::from_durations(&traffic.earlist_arrival_times());
        let methods = methods
            .iter()
            .map(|(m, s)| {
                let sets = s.scheduled_entering_times();
                (m.clone(), Timing::from_durations(&sets))
            })
            .collect();
        Self { traffic, methods }
    }
}
