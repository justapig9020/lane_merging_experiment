mod scheduler;
mod traffic_gen;

use traffic_gen::Traffic;

fn main() {
    let lamdba = 1.0;
    let count = 5;
    println!("{:?}", Traffic::new(lamdba, count));
}
