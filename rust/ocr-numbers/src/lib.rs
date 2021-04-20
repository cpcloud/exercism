#![feature(map_into_keys_values)]
// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use enumset::EnumSet;
use std::{collections::BTreeMap, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(Debug, enumset::EnumSetType)]
enum Value {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Zero => "0",
                Value::One => "1",
                Value::Two => "2",
                Value::Three => "3",
                Value::Four => "4",
                Value::Five => "5",
                Value::Six => "6",
                Value::Seven => "7",
                Value::Eight => "8",
                Value::Nine => "9",
            }
        )
    }
}

fn parse(input: &str) -> String {
    let mut valid = EnumSet::<Value>::all();
    for (line_index, line) in input.split('\n').enumerate() {
        valid &= match line_index {
            0 => parse_line_one(&line),
            1 => parse_line_two(&line),
            2 => parse_line_three(&line),
            3 => {
                // the last line must be 3 spaces
                if line != "   " {
                    EnumSet::empty()
                } else {
                    continue;
                }
            }
            index => {
                panic!("more than 4 lines: {}", index + 1);
            }
        };
    }

    valid
        .into_iter()
        .next()
        .map_or_else(|| "?".into(), |value| value.to_string())
}

fn parse_line_one(line: &str) -> EnumSet<Value> {
    match line {
        "   " => Value::One | Value::Four,
        " _ " => EnumSet::all() - (Value::One | Value::Four),
        _ => EnumSet::empty(),
    }
}

fn parse_line_two(line: &str) -> EnumSet<Value> {
    match line {
        "| |" => EnumSet::only(Value::Zero),
        "  |" => Value::One | Value::Seven,
        " _|" => Value::Two | Value::Three,
        "|_|" => Value::Four | Value::Eight | Value::Nine,
        "|_ " => Value::Five | Value::Six,
        _ => EnumSet::empty(),
    }
}

fn parse_line_three(s: &str) -> EnumSet<Value> {
    match s {
        "|_|" => Value::Zero | Value::Six | Value::Eight,
        "  |" => Value::One | Value::Four | Value::Seven,
        "|_ " => EnumSet::only(Value::Two),
        " _|" => Value::Three | Value::Five | Value::Nine,
        _ => EnumSet::empty(),
    }
}

fn parse_numbers_from_line_group(line_group: &str) -> Result<Vec<String>, Error> {
    let mut map = BTreeMap::new();

    for line in line_group.split('\n') {
        let nchars = line.len();

        if nchars % 3 != 0 {
            return Err(Error::InvalidColumnCount(nchars));
        }

        for (i, chunk) in line.as_bytes().chunks(3).enumerate() {
            let bytes = map.entry(i).or_insert_with(Vec::new);
            bytes.extend(chunk);
            bytes.push(b'\n');
        }
    }

    Ok(map
        .into_values()
        .map(|bytes| {
            String::from_utf8(bytes)
                .map(|s| s.trim_end_matches('\n').to_owned())
                .unwrap()
        })
        .collect())
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut num_newlines = 0;
    let lines = input
        .split(|c| {
            let is_newline = c == '\n';
            num_newlines += usize::from(is_newline);
            is_newline && num_newlines % 4 == 0
        })
        .map(|line_group_text| {
            Ok(parse_numbers_from_line_group(&line_group_text)?
                .iter()
                .map(|number_text| parse(number_text))
                .collect::<Vec<_>>()
                .join(""))
        })
        .collect::<Result<Vec<_>, _>>()?;

    num_newlines += 1;
    if num_newlines % 4 != 0 {
        return Err(Error::InvalidRowCount(num_newlines));
    }

    Ok(lines.join(","))
}
