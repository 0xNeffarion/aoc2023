use std::{collections::HashSet, error::Error, vec};

const DOT: char = '.';

struct Schematic(Vec<Vec<char>>);

impl Schematic {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        if x < 0 || y < 0 {
            return None;
        }

        self.0.get(y as usize).and_then(|row| row.get(x as usize))
    }

    fn is_symbol_near(&self, x: i32, y: i32) -> bool {
        for (dx, dy) in Schematic::DIRECTIONS.iter() {
            if let Some(val) = self.get(x + dx, y + dy) {
                if Self::is_symbol(val) {
                    return true;
                }
            }
        }

        false
    }

    fn is_symbol(symbol: &char) -> bool {
        *symbol != DOT && !symbol.is_ascii_digit()
    }

    fn scan_number(&self, start_x: i32, start_y: i32) -> u32 {
        let mut number = self.get(start_x, start_y).unwrap().to_digit(10).unwrap();

        let mut x = start_x - 1;
        let mut i = 1;
        while x >= 0 && x < self.0.len() as i32 {
            if let Some(value) = self.get(x, start_y) {
                if value.is_ascii_digit() {
                    number += value.to_digit(10).unwrap() * 10u32.pow(i);
                } else {
                    break;
                }
            }

            i += 1;
            x -= 1;
        }

        let mut x = start_x + 1;
        while x >= 0 && x < self.0.len() as i32 {
            if let Some(value) = self.get(x, start_y) {
                if value.is_ascii_digit() {
                    number = number * 10 + value.to_digit(10).unwrap();
                } else {
                    break;
                }
            }

            x += 1;
        }

        number
    }

    fn scan_for_part_numbers(&self) -> Vec<u32> {
        let mut part_numbers = vec![];
        let rows = self.0.len() as i32;
        let columns = self.0[0].len() as i32;

        for y in 0..columns {
            let mut number = 0;
            let mut is_part = false;
            for x in 0..rows {
                let value = self.get(x, y).unwrap();
                if value.is_ascii_digit() {
                    number = number * 10 + value.to_digit(10).unwrap();
                    if self.is_symbol_near(x, y) {
                        is_part = true;
                    }
                } else if !value.is_ascii_digit() && number != 0 {
                    if is_part {
                        part_numbers.push(number);
                        is_part = false;
                    }

                    number = 0;
                }
            }

            if number != 0 && is_part {
                part_numbers.push(number);
            }
        }

        part_numbers
    }

    fn scan_for_gear_ratios(&self) -> Vec<u32> {
        let mut gear_numbers = vec![];
        let rows = self.0.len() as i32;
        let columns = self.0[0].len() as i32;

        for y in 0..columns {
            for x in 0..rows {
                if let Some(value) = self.get(x, y) {
                    let mut parts = HashSet::new();
                    if *value == '*' {
                        for (dx, dy) in Schematic::DIRECTIONS.iter() {
                            if let Some(val) = self.get(x + dx, y + dy) {
                                if val.is_ascii_digit() {
                                    parts.insert(self.scan_number(x + dx, y + dy));
                                }
                            }
                        }
                    }

                    if parts.len() == 2 {
                        gear_numbers.push(parts.iter().product())
                    }
                }
            }
        }

        gear_numbers
    }
}

impl TryFrom<Vec<String>> for Schematic {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let values = value
            .iter()
            .map(|line| line.trim().chars().collect())
            .collect();

        Ok(Self(values))
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = crate::read_input_lines(3, 1);

    let schematic = Schematic::try_from(input)?;
    part_1(&schematic)?;
    part_2(&schematic)?;

    Ok(())
}

fn part_1(schematic: &Schematic) -> Result<(), Box<dyn Error>> {
    let part_numbers = schematic.scan_for_part_numbers();
    let sum: u32 = part_numbers.iter().sum();
    println!("1. Sum: {}", sum);

    Ok(())
}

fn part_2(schematic: &Schematic) -> Result<(), Box<dyn Error>> {
    let gears = schematic.scan_for_gear_ratios();
    let sum: u32 = gears.iter().sum();
    println!("2. Sum: {}", sum);

    Ok(())
}
