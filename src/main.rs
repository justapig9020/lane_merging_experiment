mod report;
mod scheduler;
mod traffic_gen;

use std::collections::HashMap;

use report::Report;
use scheduler::Scheduler;
use scheduler::DP;
use traffic_gen::Traffic;

use crate::scheduler::FCFS;
use crate::traffic_gen::Parameters;

fn main() {
    let para = Parameters::default();

    let schedulers: Vec<&dyn Scheduler> = vec![&DP, &FCFS];

    for l in 1..=1 {
        let lambda = l as f32 * 0.1;
        let count = 100;
        let traffic = Traffic::generate(lambda, count);
        let mut schedules = HashMap::new();
        for schd in schedulers.iter() {
            let sol = schd.solve(&traffic, &para).unwrap();
            schedules.insert(schd.method(), sol);
        }
        let report = Report::new(&traffic, &schedules);
        println!("{}", serde_json::to_string(&report).unwrap());
    }
}
