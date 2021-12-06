use std::fs;

fn main() {
    let raw = fs::read_to_string("assets/day06.txt").expect("could not open file!");
    // let raw = "3,4,3,1,2";
    let ages: Vec<usize> = raw.split(',').map(|x| x.parse().unwrap()).collect();
    dbg!(laterfish_count(&ages, 80));
    dbg!(laterfish_count(&ages, 256));
}

fn laterfish_count(ages: &[usize], days: usize) -> usize {
    let mut counts = [0; 9];
    for age in ages {
        counts[*age] += 1;
    }

    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    counts.iter().sum()
}
