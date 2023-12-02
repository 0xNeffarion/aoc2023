use std::{collections::BTreeMap, error::Error};

use map_macro::hash_map;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = crate::read_input_lines(1, 1);

    part_1(&input)?;
    part_2(&input)?;

    Ok(())
}

fn part_1(input: &[String]) -> Result<(), Box<dyn Error>> {
    let sum = input
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|x| (*x.first().unwrap_or(&0) * 10) + x.last().unwrap_or(&0))
        .sum::<u32>();

    println!("1. Sum: {}", sum);

    Ok(())
}

fn part_2(input: &[String]) -> Result<(), Box<dyn Error>> {
    let digits = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    let sum = input
        .iter()
        .map(|line| {
            let mut indices = BTreeMap::new();
            let mut indices_inverse = BTreeMap::new();
            for (word, digit) in digits.iter() {
                let digit_str = &digit.to_string();

                if let Some(index) = line.find(word) {
                    indices.insert(index, *digit);
                }

                if let Some(index) = line.find(digit_str) {
                    indices.insert(index, *digit);
                }

                if let Some(index) = line.rfind(word) {
                    indices_inverse.insert(index, *digit);
                }

                if let Some(index) = line.rfind(digit_str) {
                    indices_inverse.insert(index, *digit);
                }
            }

            (indices, indices_inverse)
        })
        .map(|(x, z)| (x.first_key_value().unwrap().1 * 10) + z.last_key_value().unwrap().1)
        .sum::<usize>();

    println!("2. Sum: {}", sum);

    Ok(())
}
