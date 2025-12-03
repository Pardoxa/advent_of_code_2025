use std::{env::args, fs::OpenOptions, io::Read};

fn main() {
    let mut args = args();
    let input_file = args.next_back().unwrap();
    let mode = args.next_back().unwrap();

    let mut reader = OpenOptions::new().read(true)
        .open(input_file)
        .unwrap();
    let mut input = String::new();
    reader.read_to_string(&mut input)
        .unwrap();

    match mode.as_str() 
    {
        "1" => puzzle1(&input),
        "2" => puzzle2(&input),
        _ => unimplemented!()
    }
}


pub fn puzzle1(input: &str)
{
    let mut sum = 0;

    let iter = input.split('\n');

    for line in iter {
        let number = largest2(line);
        println!("{number}");
        sum += number;
    }
    println!("{sum}");
}

pub fn puzzle2(input: &str)
{
    let mut sum = 0;

    let iter = input.split('\n');

    for line in iter {
        let number = largest_n(line, 12);
        sum += number;
    }
    println!("{sum}");
}


pub fn largest_n(line: &str, n: u32) -> u64
{
    let mut factor = 10_u64.pow(n - 1);
    let mut sum = 0;
    let mut index_offset = 0;
    let len = line.len();
    
        
    for untouched in (0..n as usize).rev()
    {
        let slice= &line[index_offset..len - untouched];
    
        let mut iter = slice.chars().enumerate();
        let (mut idx_largest, mut char_largest) = iter.next().unwrap();

        for (idx, char) in iter
        {
            if char_largest < char {
                char_largest = char;
                idx_largest = idx;
            }
        }
        index_offset += idx_largest + 1;
        sum += factor * to_number(char_largest);
        factor /= 10;
    }
    sum
}




pub fn largest2(line: &str) -> u64
{
    let len = line.len();
    let (slice, _) = line.split_at(len - 1);
    
    let mut iter = slice.chars().enumerate();
    let (mut idx_largest, mut char_largest) = iter.next().unwrap();

    for (idx, char) in iter
    {
        if char_largest < char {
            char_largest = char;
            idx_largest = idx;
        }
    }

    let rest = &line[idx_largest + 1..];
    let max = rest.chars().max().unwrap();
    
    to_number(char_largest) * 10 + to_number(max)
}

pub fn to_number(number: char) -> u64
{
    match number
    {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!()
    }
}