use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[test]
fn numbers_are_extracted_from_string() {
    assert_eq!(extract_number_from_string("1abc2"), 12);
    assert_eq!(extract_number_from_string("pqr3stu8vwx"), 38);
    assert_eq!(extract_number_from_string("a1b2c3d4e5f"), 15);
    assert_eq!(extract_number_from_string("treb7uchet"), 77);
}

fn extract_number_from_string(text: &str) -> u32 {
    let numbers = text.matches(char::is_numeric).collect::<Vec<&str>>();

    let first = numbers.first().map_or(0, |c| c.parse::<u32>().unwrap());
    let last = numbers.last().map_or(0, |c| c.parse::<u32>().unwrap());

    first * 10 + last
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let number = extract_number_from_string(&line.unwrap());
        sum += number;
    }

    println!("{}", sum);

    Ok(())
}
