use std::env::args;

fn main() {
    let input = include_str!("../puzzle_input.dat");

    match args().next_back().unwrap().as_str() {
        "1" => puzzle1(input),
        "2" => puzzle2(input),
        _ => unimplemented!(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    Paper,
    Nothing,
}

pub fn puzzle1(input: &str) {
    let lagerhalle: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Space::Paper,
                    '.' => Space::Nothing,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut helper: Vec<_> = lagerhalle
        .iter()
        .map(|line| vec![0_u8; line.len()])
        .collect();

    for (line_idx, line) in lagerhalle.iter().enumerate() {
        for (row_idx, element) in line.iter().enumerate() {
            if *element == Space::Paper {
                if let Some(line_m1) = line_idx.checked_sub(1) {
                    let line = &mut helper[line_m1];
                    let rows = if row_idx == 0 {
                        &mut line[0..=1]
                    } else {
                        let len = line.len() - 1;
                        &mut line[row_idx - 1..=(row_idx + 1).min(len)]
                    };
                    rows.iter_mut().for_each(|e| *e += 1);
                }

                let line = &mut helper[line_idx];
                let rows = if row_idx == 0 {
                    &mut line[0..=1]
                } else {
                    let len = line.len() - 1;
                    &mut line[row_idx - 1..=(row_idx + 1).min(len)]
                };
                rows.iter_mut().for_each(|e| *e += 1);

                if let Some(line) = helper.get_mut(line_idx + 1) {
                    let rows = if row_idx == 0 {
                        &mut line[0..=1]
                    } else {
                        let len = line.len() - 1;
                        &mut line[row_idx - 1..=(row_idx + 1).min(len)]
                    };
                    rows.iter_mut().for_each(|e| *e += 1);
                }
            }
        }
    }

    let mut sum = 0;

    for (helper_line, lager_line) in helper.into_iter().zip(lagerhalle) {
        for (r, lager) in helper_line.into_iter().zip(lager_line) {
            if lager == Space::Paper && r <= 4 {
                sum += 1;
            }
        }
    }

    println!("{sum}")
}

pub fn puzzle2(input: &str) {
    let mut lagerhalle: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Space::Paper,
                    '.' => Space::Nothing,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut helper: Vec<_> = lagerhalle
        .iter()
        .map(|line| vec![0_u8; line.len()])
        .collect();

    let mut sum = 0;
    loop {
        let removed = remove_and_sum(&mut helper, &mut lagerhalle);
        if removed == 0 {
            break;
        }
        sum += removed;
    }

    println!("{sum}")
}

pub fn remove_and_sum(helper: &mut [Vec<u8>], lagerhalle: &mut [Vec<Space>]) -> usize {
    for (line_idx, line) in lagerhalle.iter().enumerate() {
        for (row_idx, element) in line.iter().enumerate() {
            if *element == Space::Paper {
                if let Some(line_m1) = line_idx.checked_sub(1) {
                    let line = &mut helper[line_m1];
                    let rows = if row_idx == 0 {
                        &mut line[0..=1]
                    } else {
                        let len = line.len() - 1;
                        &mut line[row_idx - 1..=(row_idx + 1).min(len)]
                    };
                    rows.iter_mut().for_each(|e| *e += 1);
                }

                let line = &mut helper[line_idx];
                let rows = if row_idx == 0 {
                    &mut line[0..=1]
                } else {
                    let len = line.len() - 1;
                    &mut line[row_idx - 1..=(row_idx + 1).min(len)]
                };
                rows.iter_mut().for_each(|e| *e += 1);

                if let Some(line) = helper.get_mut(line_idx + 1) {
                    let rows = if row_idx == 0 {
                        &mut line[0..=1]
                    } else {
                        let len = line.len() - 1;
                        &mut line[row_idx - 1..=(row_idx + 1).min(len)]
                    };
                    rows.iter_mut().for_each(|e| *e += 1);
                }
            }
        }
    }

    let mut sum = 0;

    for (helper_line, lager_line) in helper.iter_mut().zip(lagerhalle) {
        for (r, lager) in helper_line.iter_mut().zip(lager_line) {
            if *lager == Space::Paper && *r <= 4 {
                sum += 1;
                *lager = Space::Nothing;
            }
            *r = 0;
        }
    }
    sum
}
