use std::env::args;

fn main() {
    let input = include_str!("../puzzle_input.dat");

    let args = args().next_back().unwrap();

    match args.as_str() {
        "1" => puzzle1(input),
        "2" => puzzle2(input.to_owned()),
        _ => unimplemented!(),
    }
}

pub fn puzzle2(mut input: String) {
    // Einmal die ganze Reihenfolge umdrehen, damit wir nicht ständig von hinten
    // lesen müssen und damit mein Gehirn keinen Knoten bekommt.
    // Da wir nur ASCII character haben, ist das hier in ordnung,
    // eigentlich müsste man das hier asserten oder so, but who cares
    unsafe { input.as_bytes_mut() }.reverse();

    let mut iter = input.split('\n');
    // Durch das reversen ist meine erste line nun die mit den operators
    let operators = to_operators(iter.next().unwrap());
    // Der rest kommt in die Matrix, wobei ich die Reiehenfolge nochmal so shuffle
    // wie sie für mich Sinn ergibt
    let matrix: Vec<_> = iter.rev().map(|line| line.as_bytes()).collect();

    let len = matrix[0].len();

    let mut sum = 0;
    let mut operator = operators.iter();

    let mut current_operation = operator.next().unwrap().function();
    let mut current_calculation = None;

    for j in 0..len {
        let mut number: u64 = 0;
        let mut exponent = 0;
        for i in (0..matrix.len()).rev() {
            let digit = match matrix[i][j] {
                b' ' => continue,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                _ => unreachable!(),
            };
            number += digit * 10_u64.pow(exponent);
            exponent += 1;
        }
        if exponent == 0 {
            current_operation = operator.next().unwrap().function();

            sum += current_calculation.take().unwrap();
        } else {
            current_calculation = Some(
                current_calculation
                    .map(|current| current_operation(current, number))
                    .unwrap_or(number),
            );
        }
    }
    sum += current_calculation.take().unwrap();

    println!("{sum}")
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn function(self) -> fn(u64, u64) -> u64 {
        match self {
            Self::Add => std::ops::Add::add,
            Self::Multiply => std::ops::Mul::mul,
        }
    }
}

pub fn to_operators(line: &str) -> Vec<Operator> {
    line.split_ascii_whitespace()
        .map(|operator| match operator {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            other => {
                println!("{other}");
                panic!()
            }
        })
        .collect()
}

pub fn puzzle1(input: &str) {
    let mut iter = input.split('\n');
    let operators = iter.next_back().unwrap();

    let operators = to_operators(operators);

    let mut numbers: Vec<u64> = iter
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    for line in iter {
        for ((next_number, current_number), operator) in line
            .split_ascii_whitespace()
            .zip(&mut numbers)
            .zip(&operators)
        {
            let next_number: u64 = next_number.parse().unwrap();
            match operator {
                Operator::Add => {
                    *current_number += next_number;
                }
                Operator::Multiply => {
                    *current_number *= next_number;
                }
            }
        }
    }

    let result: u64 = numbers.iter().sum();

    println!("{result}")
}
