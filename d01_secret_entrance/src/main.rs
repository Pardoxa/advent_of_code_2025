use std::{env::args, fs::{OpenOptions}, io::{BufRead, BufReader}};

fn main() {
    let mut args = args();
    let input_file = args.next_back().unwrap();
    let mode = args.next_back().unwrap();

    let reader = OpenOptions::new().read(true)
        .open(input_file)
        .unwrap();
    let buf = BufReader::new(reader);

    
    let lines = buf.lines()
        .map(|r| r.unwrap());

    let password = match mode.as_str() 
    {
        "1" => puzzle1(lines),
        "2" => puzzle2(lines),
        _ => unimplemented!()
    };

    println!("{password}")
    
}

pub fn puzzle1<I: Iterator<Item=String>>(lines: I) -> usize
{
    let mut position = 50_u8;

    let mut counter = 0;

    for line in lines
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
        position += rotation;
        position %= 100;
        if position == 0 {
            counter += 1;
        }
    }

    counter
}

pub fn puzzle2<I: Iterator<Item=String>>(lines: I) -> usize
{
    let mut position = 50_u8;

    let mut counter = 0;

    for line in lines
    {
        let number = &line[1..];
        let full_rotations = number.parse::<usize>().unwrap() / 100;   
        counter += full_rotations;
        if line.starts_with('L')
        {
            let start = number.len().saturating_sub(2);
            let left = number[start..].parse::<u8>().unwrap();
            if position != 0 && left > position {
                counter += 1;
            }
            position = (position + 100 - left) % 100;
            
        } else {
            let start = number.len().saturating_sub(2);
            let right: u8 = number[start..].parse().unwrap();
            let rotated_position = right + position;
            if rotated_position > 100 {
                counter += 1;
            }
            position = rotated_position % 100;
        };
        

        if position == 0 {
            counter += 1;
        }
    }

    counter
}
