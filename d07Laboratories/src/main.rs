use std::env::args;

fn main() {
    let input = include_str!("../puzzle_input.dat");

    let mode = args().next_back().unwrap();

    match mode.as_str() {
        "1" => puzzle1(input),
        "2" => puzzle2(input),
        _ => unimplemented!(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Empty,
    Splitter,
    ChargedSplitter,
    TractorBeam,
}

impl Space {
    pub fn hit_with_tractor_beam(&mut self) {
        match self {
            Self::Empty => *self = Self::TractorBeam,
            Self::Splitter => *self = Self::ChargedSplitter,
            Self::ChargedSplitter | Self::TractorBeam => (),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Space2 {
    Empty,
    Splitter,
    ChargedSplitter(u64), // how large is the charge?
    TractorBeam(u64),     // counting number of paths
}

impl Space2 {
    pub fn hit_with_tractor_beam(&mut self, charge: u64) {
        match self {
            Self::Empty => *self = Self::TractorBeam(charge),
            Self::Splitter => *self = Self::ChargedSplitter(charge),
            Self::ChargedSplitter(c) | Self::TractorBeam(c) => *c += charge,
        }
    }
}

pub fn puzzle1(input: &str) {
    let len = input.lines().next().unwrap().len();
    let mut height = 0;
    let mut matrix: Vec<_> = input
        .lines()
        .flat_map(|line| {
            height += 1;
            line.as_bytes().iter().map(|&pixel| match pixel {
                b'.' => Space::Empty,
                b'^' => Space::Splitter,
                b'S' => Space::TractorBeam,
                _ => unreachable!(),
            })
        })
        .collect();

    let mut iter = matrix.chunks_exact_mut(len);

    let mut higher_line = iter.next().unwrap();

    let mut split_counter = 0_u64;

    for lower_line in iter {
        // First only search for charged splitters and duplicate the beams if applicable
        for i in 0..len {
            if higher_line[i] == Space::ChargedSplitter {
                if i > 0 {
                    let left = &mut higher_line[i - 1];
                    if *left == Space::Empty {
                        *left = Space::TractorBeam;
                    }
                }

                if let Some(right) = higher_line.get_mut(i + 1)
                    && *right == Space::Empty
                {
                    *right = Space::TractorBeam;
                }
                split_counter += 1;
            }
        }

        for (idx, space) in higher_line.iter().enumerate() {
            if *space == Space::TractorBeam {
                lower_line[idx].hit_with_tractor_beam();
            }
        }

        higher_line = lower_line;
    }

    for line in matrix.chunks_exact(len) {
        for space in line {
            let c = match space {
                Space::ChargedSplitter => 'C',
                Space::Empty => '.',
                Space::TractorBeam => 'T',
                Space::Splitter => 'N',
            };
            print!("{c}");
        }
        println!()
    }

    println!("{split_counter}")
}

pub fn puzzle2(input: &str) {
    let len = input.lines().next().unwrap().len();
    let mut height = 0;
    let mut matrix: Vec<_> = input
        .lines()
        .flat_map(|line| {
            height += 1;
            line.as_bytes().iter().map(|&pixel| match pixel {
                b'.' => Space2::Empty,
                b'^' => Space2::Splitter,
                b'S' => Space2::TractorBeam(1),
                _ => unreachable!(),
            })
        })
        .collect();

    let mut iter = matrix.chunks_exact_mut(len);

    let mut higher_line = iter.next().unwrap();

    for lower_line in iter {
        // First only search for charged splitters and duplicate the beams if applicable
        for i in 0..len {
            if let Space2::ChargedSplitter(charge) = higher_line[i] {
                if i > 0 {
                    let left = &mut higher_line[i - 1];
                    match left {
                        Space2::Empty => *left = Space2::TractorBeam(charge),
                        Space2::TractorBeam(current) => *current += charge,
                        _ => unreachable!(),
                    }
                }

                if let Some(right) = higher_line.get_mut(i + 1) {
                    match right {
                        Space2::Empty => *right = Space2::TractorBeam(charge),
                        Space2::TractorBeam(current) => *current += charge,
                        _ => unreachable!(),
                    }
                }
            }
        }

        for (idx, space) in higher_line.iter().enumerate() {
            if let Space2::TractorBeam(charge) = *space {
                lower_line[idx].hit_with_tractor_beam(charge);
            }
        }

        higher_line = lower_line;
    }

    let sum: u64 = matrix
        .chunks_exact(len)
        .last()
        .unwrap()
        .iter()
        .map(|space| {
            if let Space2::TractorBeam(charge) = space {
                *charge
            } else {
                0
            }
        })
        .sum();

    println!("{sum}")
}
