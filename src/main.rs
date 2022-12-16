mod report;
mod scheduler;
mod traffic_gen;

use std::collections::HashMap;

use clap::Parser;
use report::Report;
use scheduler::Scheduler;
use scheduler::DP;
use traffic_gen::Traffic;

use crate::scheduler::SimulationAnnealing;
use crate::scheduler::FCFS;
use crate::traffic_gen::Parameters;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    n: usize,
    #[arg(short, long)]
    lambda: f32,
    #[arg(short, long, default_value_t = 1)]
    times: usize,
}

fn main() {
    let args = Args::parse();

    let para = Parameters::default();
    let sa = SimulationAnnealing::new(50, 50);
    let schedulers: Vec<&dyn Scheduler> = vec![&DP, &FCFS, &sa];
    let mut report = Vec::new();

    let count = args.n;
    let lambda = args.lambda;
    let times = args.times;
    for _ in 0..times {
        let traffic = Traffic::generate(lambda, count);
        let mut schedules = HashMap::new();
        for schd in schedulers.iter() {
            let sol = schd.solve(&traffic, &para).unwrap();
            schedules.insert(schd.method(), sol);
        }
        report.push(Report::new(&para, lambda, count, &traffic, &schedules));
    }
    println!("{}", serde_json::to_string(&report).unwrap());
}
