use std::{error::Error, str::FromStr};

const MAX_GREEN_CUBES: u32 = 13;
const MAX_RED_CUBES: u32 = 12;
const MAX_BLUE_CUBES: u32 = 14;

struct GameSet {
    green_cubes: u32,
    red_cubes: u32,
    blue_cubes: u32,
}

impl GameSet {
    fn is_valid(&self) -> bool {
        self.green_cubes <= MAX_GREEN_CUBES
            && self.red_cubes <= MAX_RED_CUBES
            && self.blue_cubes <= MAX_BLUE_CUBES
    }
}

struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn power(&self) -> u32 {
        let max_reds = self.sets.iter().map(|x| x.red_cubes).max().unwrap_or(0);
        let max_blues = self.sets.iter().map(|x| x.blue_cubes).max().unwrap_or(0);
        let max_greens = self.sets.iter().map(|x| x.green_cubes).max().unwrap_or(0);

        max_reds * max_blues * max_greens
    }
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut split = value.split(':');
        let id = split
            .next()
            .ok_or("No game id found")?
            .replace("Game ", "")
            .trim()
            .parse::<u32>()?;

        let mut sets = Vec::new();
        for set in split.next().ok_or("No sets found")?.split(';') {
            let split = set.split(',');
            let mut green_cubes = 0;
            let mut red_cubes = 0;
            let mut blue_cubes = 0;

            for cube in split {
                if cube.contains("green") {
                    green_cubes = cube.replace("green", "").trim().parse::<u32>()?;
                }

                if cube.contains("red") {
                    red_cubes = cube.replace("red", "").trim().parse::<u32>()?;
                }

                if cube.contains("blue") {
                    blue_cubes = cube.replace("blue", "").trim().parse::<u32>()?;
                }
            }

            sets.push(GameSet {
                green_cubes,
                red_cubes,
                blue_cubes,
            });
        }

        Ok(Game { id, sets })
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = crate::read_input_lines(2, 1);

    let mut games = Vec::new();
    for line in input {
        let game = Game::from_str(&line)?;
        games.push(game);
    }

    part_1(&games)?;
    part_2(&games)?;

    Ok(())
}

fn part_1(games: &[Game]) -> Result<(), Box<dyn Error>> {
    println!(
        "1. Sum: {}",
        games
            .iter()
            .filter(|game| game.sets.iter().all(|set| set.is_valid()))
            .map(|game| game.id)
            .sum::<u32>()
    );

    Ok(())
}

fn part_2(games: &[Game]) -> Result<(), Box<dyn Error>> {
    println!(
        "2. Sum: {}",
        games.iter().map(|game| game.power()).sum::<u32>()
    );

    Ok(())
}
