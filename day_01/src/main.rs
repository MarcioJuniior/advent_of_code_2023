use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod iterator;

use crate::iterator::StringNumbers;

#[test]
fn numbers_are_extracted_from_string_digits() {
    assert_eq!(extract_number_from_string_digits("1abc2"), 12);
    assert_eq!(extract_number_from_string_digits("pqr3stu8vwx"), 38);
    assert_eq!(extract_number_from_string_digits("a1b2c3d4e5f"), 15);
    assert_eq!(extract_number_from_string_digits("treb7uchet"), 77);
}

fn extract_number_from_string_digits(text: &str) -> u32 {
    let numbers = text.matches(char::is_numeric).collect::<Vec<&str>>();

    let first = numbers.first().map_or(0, |c| c.parse::<u32>().unwrap());
    let last = numbers.last().map_or(0, |c| c.parse::<u32>().unwrap());

    first * 10 + last
}

#[test]
fn numbers_are_extracted_from_string_digits_or_text() {
    assert_eq!(extract_number_from_string_digits_or_text("two1nine"), 29);
    assert_eq!(extract_number_from_string_digits_or_text("eightwothree"), 83);
    assert_eq!(extract_number_from_string_digits_or_text("abcone2threexyz"), 13);
    assert_eq!(extract_number_from_string_digits_or_text("xtwone3four"), 24);
    assert_eq!(extract_number_from_string_digits_or_text("4nineeightseven2"), 42);
    assert_eq!(extract_number_from_string_digits_or_text("zoneight234"), 14);
    assert_eq!(extract_number_from_string_digits_or_text("7pqrstsixteen"), 76);
    assert_eq!(extract_number_from_string_digits_or_text("twone"), 21);
}

fn extract_number_from_string_digits_or_text(text: &str) -> u32 {
    let numbers = StringNumbers::new(text.to_string()).collect::<Vec<u32>>();

    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();

    first * 10 + last
}

fn part1(reader: BufReader<File>) -> u32 {
    reader.lines()
        .map(|line| extract_number_from_string_digits(&line.unwrap()))
        .sum()
}

fn part2(reader: BufReader<File>) -> u32 {
    reader.lines()
        .map(|line| extract_number_from_string_digits_or_text(&line.unwrap()))
        .sum()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let part = &args[2];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let result = match part.as_str() {
        "1" => Ok(part1(reader)),
        "2" => Ok(part2(reader)),
        &_ => Err("Unsupported Operation"),
    };

    println!("{}", result.unwrap());

    Ok(())
}
