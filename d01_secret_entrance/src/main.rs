use std::{env::args, fs::{OpenOptions}, io::{BufRead, BufReader}};

fn main() {
    let input_file = args().next_back().unwrap();
    let reader = OpenOptions::new().read(true)
        .open(input_file)
        .unwrap();
    let buf = BufReader::new(reader);

    let mut lines = buf.lines();

    let mut position = 50_u8;

    let mut counter = 0;

    while let Some(Ok(line)) = lines.next()
    {
        let rotation: u8 = if let Some(left) = line.strip_prefix('L')
        {
            let start = left.len().saturating_sub(2);
            let left = &left[start..];
            100 - left.parse::<u8>().unwrap()
        } else if let Some(right) = line.strip_prefix('R')
        {
            let start = right.len().saturating_sub(2);
            let right = &right[start..];
            right.parse().unwrap()
        } else {
            unreachable!()
        };
        position = position.checked_add(rotation).unwrap();
        position %= 100;
        if position == 0 {
            counter += 1;
        }
    }

    println!("{counter}")
}


