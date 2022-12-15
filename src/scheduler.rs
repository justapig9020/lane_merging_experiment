use std::time::Duration;

use itertools::Itertools;

use crate::traffic_gen::Lane;
use crate::traffic_gen::Parameters;
use crate::traffic_gen::Traffic;

const A: usize = 0;
const B: usize = 1;

pub trait Scheduler {
    fn solve(traffic: &Traffic, para: &Parameters) -> Option<Schedule>;
}

#[derive(Debug)]
pub struct Schedule {
    scheduled_entering_times: Vec<Lane>,
    total_time: Duration,
}

impl Schedule {
    pub fn total_time(&self) -> Duration {
        self.total_time
    }
    pub fn scheduled_entering_times(&self) -> Vec<&[Duration]> {
        self.scheduled_entering_times
            .iter()
            .map(|set| set.times())
            .collect_vec()
    }
}

pub struct DP;

impl Scheduler for DP {
    fn solve(traffic: &Traffic, para: &Parameters) -> Option<Schedule> {
        let eats = traffic.earlist_arrival_times();
        let a = eats.get(0)?;
        println!("{a:?}");
        let b = eats.get(1)?;
        println!("{b:?}");
        let alpha = a.len();
        let beta = b.len();
        let w_e = para.w_e; // W= in paper
        let w_p = para.w_p; // W+ in paper

        // l(i, j, A) => i and j vehicle in passed vehicles are from A lane and B lane, the last vehicle passed the merging point is from A lane
        let mut l = vec![vec![vec![Duration::default(); 2]; beta + 1]; alpha + 1];

        // Record previous situation of each posible solutions
        let mut prev = vec![vec![vec![0; 2]; beta + 1]; alpha + 1];

        l[0][0][A] = Duration::from_secs(0);
        l[0][0][B] = Duration::from_secs(0);

        l[1][0][A] = a[0];
        l[0][1][B] = b[0];

        // If all of the passed vehicles are from A lane
        for i in 2..=alpha {
            l[i][0][A] = Duration::max(a[i - 1], l[i - 1][0][A] + w_e);
            prev[i][0][A] = A;
        }

        // If all of the passed vehicles are from B lane
        for j in 2..=alpha {
            l[0][j][B] = Duration::max(b[j - 1], l[0][j - 1][B] + w_e);
            prev[0][j][B] = B;
        }

        // Consider all of the posible scheduling
        for i in 1..=alpha {
            for j in 1..=beta {
                // There are two posibles of l(i, j)
                // 1. The last vehicle is from A
                // 2. The last vehicle is from B
                // We have to record all of these posibles
                // We evalute both posible from there all of the prior posibles

                // The passing vehicle is from lane A
                /*
                println!("a: {:?}", a[i - 1]);
                println!("l: {:?}", l[i - 1][j][A]);
                println!("w: {:?}", w_e);
                println!("l + w: {:?}", l[i - 1][j][A] + w_e);
                 */
                let prev_from_a = if i - 1 == 0 {
                    Duration::MAX
                } else {
                    Duration::max(a[i - 1], l[i - 1][j][A] + w_e)
                };
                //println!("prev: {:?}", prev_from_a);
                let prev_from_b = if j == 0 {
                    Duration::MAX
                } else {
                    Duration::max(a[i - 1], l[i - 1][j][B] + w_p)
                };
                (l[i][j][A], prev[i][j][A]) = if prev_from_a <= prev_from_b {
                    (prev_from_a, A)
                } else {
                    (prev_from_b, B)
                };

                // The passing vehicle is from lane B
                let prev_from_a = if i == 0 {
                    Duration::MAX
                } else {
                    Duration::max(b[j - 1], l[i][j - 1][A] + w_p)
                };
                let prev_from_b = if j - 1 == 0 {
                    Duration::MAX
                } else {
                    Duration::max(b[j - 1], l[i][j - 1][B] + w_e)
                };

                (l[i][j][B], prev[i][j][B]) = if prev_from_a <= prev_from_b {
                    (prev_from_a, A)
                } else {
                    (prev_from_b, B)
                };
            }
        }
        for k in l.iter() {
            for p in k.iter() {
                print!("{:?} ", p[0]);
            }
            println!("");
        }
        for k in l.iter() {
            for p in k.iter() {
                print!("{:?} ", p[1]);
            }
            println!("");
        }
        for k in prev.iter() {
            for p in k.iter() {
                print!("{:?} ", p[0]);
            }
            println!("");
        }
        for k in prev.iter() {
            for p in k.iter() {
                print!("{:?} ", p[1]);
            }
            println!("");
        }
        let (total_time, last_from) = if l[alpha][beta][A] <= l[alpha][beta][B] {
            (l[alpha][beta][A], A)
        } else {
            (l[alpha][beta][B], B)
        };
        let mut scheduled_entering_times = vec![
            vec![Duration::default(); alpha],
            vec![Duration::default(); beta],
        ];

        let mut i = alpha;
        let mut j = beta;
        let mut passing_from = last_from;
        while i > 0 && j > 0 {
            let set = l[i][j][passing_from];
            let idx = if passing_from == A { &mut i } else { &mut j };
            scheduled_entering_times[passing_from][*idx - 1] = set;
            *idx -= 1;
            passing_from = prev[i][j][passing_from];
        }
        let scheduled_entering_times = scheduled_entering_times
            .into_iter()
            .map(|set| Lane::new(set))
            .collect_vec();
        Some(Schedule {
            scheduled_entering_times,
            total_time,
        })
    }
}
