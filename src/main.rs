mod scheduler;
mod traffic_gen;

use scheduler::Scheduler;
use scheduler::DP;
use traffic_gen::Traffic;

use crate::scheduler::FCFS;
use crate::traffic_gen::Parameters;

fn main() {
    let para = Parameters::default();

    let schedulers: Vec<&dyn Scheduler> = vec![&DP, &FCFS];

    for l in 1..=5 {
        let lambda = l as f32 * 0.1;
        let count = 100;
        let traffic = Traffic::generate(lambda, count);
        println!("Lambda: {}", lambda);
        for schd in schedulers.iter() {
            let sol = schd.solve(&traffic, &para).unwrap();
            println!("{}: {:?}", schd.method(), sol.t_last());
        }
    }
}
