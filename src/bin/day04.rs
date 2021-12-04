use ndarray::{Array1, Array2};
use std::{collections::HashSet, fs, str::FromStr};

fn main() {
    let raw = fs::read_to_string("assets/day04.txt").expect("could not open file!");
    let mut sections = raw.split("\n\n");
    let drawn: Vec<usize> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards: Vec<Board> = sections.map(|s| s.parse().unwrap()).collect();

    dbg!(part_a(&boards, &drawn).unwrap());
    dbg!(part_b(&boards, &drawn).unwrap());
}

fn part_a(boards: &[Board], drawn: &[usize]) -> Option<usize> {
    let mut boards = boards.to_vec();
    for cur_drawn in drawn {
        for board in &mut boards {
            board.mark(*cur_drawn);
            if let Some(score) = board.score {
                return Some(score * cur_drawn);
            }
        }
    }
    None
}

fn part_b(boards: &[Board], drawn: &[usize]) -> Option<usize> {
    let mut boards = boards.to_vec();
    let mut last_score = None;
    let mut winning_boards: HashSet<usize> = HashSet::new();
    for cur_drawn in drawn {
        for (idx, board) in boards.iter_mut().enumerate() {
            board.mark(*cur_drawn);
            if let Some(score) = board.score {
                if !winning_boards.contains(&idx) {
                    last_score = Some(score * cur_drawn);
                    winning_boards.insert(idx);
                }
            }
        }
    }
    last_score
}

#[derive(Debug, Clone)]
struct Board {
    vals: Array2<(usize, bool)>,
    score: Option<usize>,
}

impl FromStr for Board {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Array1<(usize, bool)> = s
            .lines()
            .flat_map(|l| l.split_ascii_whitespace())
            .flat_map(|c| c.parse())
            .map(|x| (x, false))
            .collect();
        let vals = vals.into_shape((5, 5))?;
        Ok(Self { vals, score: None })
    }
}

impl Board {
    fn mark(&mut self, val: usize) {
        if let Some(entry) = self.vals.iter_mut().find(|x| x.0 == val) {
            entry.1 = true;
            self.calc_won();
        }
    }

    fn calc_won(&mut self) {
        let mut has_won = false;
        for row in self.vals.rows() {
            if row.iter().all(|x| x.1) {
                has_won = true;
                break;
            }
        }
        for col in self.vals.columns() {
            if col.iter().all(|x| x.1) {
                has_won = true;
                break;
            }
        }
        self.score = if has_won {
            Some(
                self.vals
                    .iter()
                    .filter_map(|x| if !x.1 { Some(x.0) } else { None })
                    .sum(),
            )
        } else {
            None
        }
    }
}
