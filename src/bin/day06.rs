use itertools::repeat_n;
use std::{collections::VecDeque, fs};

fn main() {
    let raw = fs::read_to_string("assets/day06.txt").expect("could not open file!");
    let ages: Vec<usize> = raw.split(',').map(|x| x.parse().unwrap()).collect();
    dbg!(laterfish_count(&ages, 80));
    dbg!(laterfish_count(&ages, 256));
    dbg!(laterfish_count(&ages, 1_000_000_000)); // just to test time complexity
}

fn laterfish_count(ages: &[usize], days: usize) -> usize {
    let mut counts: VecDeque<usize> = repeat_n(0, 9).collect();
    for age in ages {
        counts[*age] += 1;
    }

    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    counts.iter().sum()
}
