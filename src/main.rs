mod scheduler;
mod traffic_gen;

use scheduler::Scheduler;
use scheduler::DP;
use traffic_gen::Traffic;

use crate::traffic_gen::Parameters;

fn main() {
    let lamdba = 1.0;
    let count = 5;
    let para = Parameters::default();
    let traffic = Traffic::generate(lamdba, count);
    println!("{traffic:?}");
    let sol = DP::solve(&traffic, &para).unwrap();
    println!("{sol:?}");
}
