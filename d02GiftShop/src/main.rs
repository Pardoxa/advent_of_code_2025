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

pub fn puzzle2(input: &str)
{
    let iter = input.split(',');


    let mut sum = 0;

    for range in iter {
        let Some((from, to)) = range.split_once('-') else {
            unreachable!()
        };

        let from: u64 = from.parse().unwrap();
        let to: u64 = to.parse().unwrap();

        'mid: for to_check in from..=to
        {
            let number_str = to_check.to_string();
            let number_of_digits = number_str.len();
            'inner: for power in 1..=number_of_digits/2 {
                if number_of_digits % power == 0 {
                    // How often can I split the string?
                    let times = number_of_digits / power;
                    let mut last = &number_str[0..power];
                    
                    for i in 1..times {
                        let next = &number_str[power * i..power * (i+1)];
                        
                        if next != last {
                            continue 'inner;
                        }
                        last = next;
                    }
                    
                    sum += to_check;
                    continue 'mid;
                                    
                }
            }
        }
    }
    println!("{sum}");
}

pub fn puzzle1(input: &str)
{
    let iter = input.split(',');


    let mut sum = 0;

    for range in iter {
        let Some((from, to)) = range.split_once('-') else {
            unreachable!()
        };

        let from: u64 = from.parse().unwrap();
        let to: u64 = to.parse().unwrap();

        for to_check in from..=to
        {
            let string = to_check.to_string();
            let half = string.len() / 2;
            let (left, right) = string.split_at(half);
            if left == right {
                sum += to_check;
            }
        }
    }
    println!("{sum}");
}

