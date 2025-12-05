use std::{collections::HashSet, env::args, ops::RangeInclusive};

fn main() {
    let input = include_str!("../input.dat");

    let mode = args().next_back().unwrap();

    match mode.as_str() {
        "1" => puzzle1(input),
        "2" => puzzle2(input),
        _ => unimplemented!(),
    }
}

pub fn puzzle1(input: &str) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            let left = left.parse::<u64>().unwrap();
            let right = right.parse().unwrap();
            left..=right
        })
        .collect();

    let mut sum = 0;

    for id in ids.lines() {
        let id = id.parse().unwrap();
        for range in ranges.iter() {
            if range.contains(&id) {
                sum += 1;
                break;
            }
        }
    }

    println!("{sum}")
}

pub fn puzzle2(input: &str) {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut range_collection: HashSet<RangeInclusive<u64>> = HashSet::new();

    for range in ranges.lines() {
        let (left, right) = range.split_once('-').unwrap();
        let left = left.parse::<u64>().unwrap();
        let right = right.parse().unwrap();
        let range = left..=right;

        let mut to_remove = Vec::new();
        let mut to_add = range;

        for stored in range_collection.iter() {
            if stored.contains(&left)
                || stored.contains(&right)
                || to_add.contains(stored.start())
                || to_add.contains(stored.end())
            {
                to_add = (*stored.start().min(to_add.start()))..=(*stored.end().max(to_add.end()));
                to_remove.push(stored.clone());
            }
        }

        for remove in to_remove {
            range_collection.remove(&remove);
        }

        range_collection.insert(to_add);
    }

    let mut sum = 0;
    for range in range_collection.iter() {
        sum += 1 + range.end() - range.start();
    }
    println!("{sum}")
}
