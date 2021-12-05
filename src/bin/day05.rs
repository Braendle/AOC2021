use anyhow::Context;
use itertools::Itertools;
use std::{fs, str::FromStr};

fn main() {
    let raw = fs::read_to_string("assets/day05.txt").expect("could not open file!");
    let lines: Vec<Line> = raw.lines().map(|l| l.parse().unwrap()).collect();
    let lines_filtered: Vec<_> = lines
        .iter()
        .filter(|l| l.is_horz_or_vert())
        .cloned()
        .collect();

    dbg!(count_overlaps(&lines_filtered));
    dbg!(count_overlaps(&lines));
}

fn count_overlaps(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(|l| l.covered_points())
        .counts()
        .iter()
        .filter(|e| *e.1 >= 2)
        .count()
}

#[derive(Debug, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1, y1, x2, y2) = s
            .split(" -> ")
            .flat_map(|p| p.split(','))
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .context("unable to unpack tuple")?;
        Ok(Self { x1, y1, x2, y2 })
    }
}

// Let's use a trait object to return iterators of different types :)
fn range_vec(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if start <= end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

impl Line {
    fn is_horz_or_vert(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
    fn covered_points(&self) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 {
            range_vec(self.y1, self.y2).map(|y| (self.x1, y)).collect()
        } else if self.y1 == self.y2 {
            range_vec(self.x1, self.x2).map(|x| (x, self.y1)).collect()
        } else {
            range_vec(self.x1, self.x2)
                .zip(range_vec(self.y1, self.y2))
                .collect()
        }
    }
}
