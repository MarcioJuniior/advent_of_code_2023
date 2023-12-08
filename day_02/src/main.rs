use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Cube {
    color: Color,
    amount: usize,
}

#[derive(Debug)]
struct Play {
    cubes: Vec<Cube>,
}

impl FromStr for Play {
    type Err = ();

    fn from_str(input: &str) -> Result<Play, Self::Err> {
        let re = Regex::new(r"(\d+) (\w+)").unwrap();

        let cubes = re
            .captures_iter(input)
            .map(|c| c.extract())
            .filter_map(|(_, [number, color])| Some((number.parse().ok()?, Color::from_str(color).ok()?)))
            .map(|(amount, color)| Cube { color, amount })
            .collect::<Vec<_>>();

        Ok(Play { cubes })
    }
}

struct Game {
    id: usize,
    configuration: HashMap<Color, usize>,
    plays: Vec<Play>,
}

impl Game {
    fn new(id: usize, plays: Vec<Play>) -> Game {
        Game {
            id,
            plays,
            configuration: HashMap::from([]),
        }
    }

    fn set_configuration(&mut self, configuration: HashMap<Color, usize>) {
        self.configuration = configuration;
    }

    fn is_possible_play(&self, play: &Play) -> bool {
        play.cubes
            .iter()
            .all(|cube| self.configuration.get(&cube.color).unwrap() >= &cube.amount)
    }

    fn is_possible(&self) -> bool {
        self.plays.iter().all(|play| self.is_possible_play(play))
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let re = Regex::new(r"Game (\d+): (.+)").unwrap();

        let (_, [id, plays]) = re.captures(input).map(|capture| capture.extract()).ok_or(())?;

        let id = id.parse::<usize>().ok().ok_or(())?;
        let plays = plays.split(";").filter_map(|str| Play::from_str(str).ok()).collect();

        Ok(Game::new(id, plays))
    }
}

#[test]
fn part1_test() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

    assert_eq!(part1(input), 8);
}

fn part1(input: &str) -> usize {
    input.lines()
        .filter_map(|line| -> Option<Game> {
            let mut game = Game::from_str(&line).ok()?;

            game.set_configuration(HashMap::from([
                (Color::Red, 12),
                (Color::Green, 13),
                (Color::Blue, 14),
            ]));

            Some(game)
        })
        .filter_map(|game| {
            match game.is_possible() {
                true => Some(game.id),
                false => None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    //println!("Part 2: {}", result.unwrap());
}
