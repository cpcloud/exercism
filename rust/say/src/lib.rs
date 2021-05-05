use std::{convert::TryFrom, str::FromStr};

fn ones(n: u64) -> Option<&'static str> {
    Some(match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => return None,
    })
}

fn teens(n: u64) -> Option<&'static str> {
    Some(match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => return None,
    })
}

fn low_numbers(n: u64) -> Option<String> {
    match n {
        1..=9 => return ones(n).map(ToOwned::to_owned),
        10..=19 => return teens(n).map(ToOwned::to_owned),
        _ => None,
    }
}

fn tens(n: u64) -> Option<&'static str> {
    Some(match n {
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => return None,
    })
}

const GROUPS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let char_chunks = n.to_string().chars().rev().collect::<Vec<char>>();
    let char_chunks = char_chunks
        .as_slice()
        .chunks(3)
        .map(|chunk| chunk.iter().copied().rev().collect::<Vec<_>>());

    let mut result = vec![];

    for (digit_chunk, &group) in char_chunks.zip(GROUPS.iter()) {
        let mut s = String::new();

        let raw_number = u64::from_str(&digit_chunk.iter().collect::<String>()).unwrap();
        let number = match digit_chunk.len() {
            3 => {
                let tens_value = raw_number - 100 * raw_number / 100;
                format!(
                    "{} hundred {}",
                    ones(&raw_number / 100).unwrap(),
                    low_numbers(tens_value).unwrap_or_else(|| {
                        let ones_value = tens_value % 10;
                        let tens_string = tens(10 * (tens_value / 10)).unwrap();
                        if ones_value != 0 {
                            format!("{}-{}", tens_string, ones(tens_value % 10).unwrap())
                        } else {
                            tens(10 * (tens_value / 10)).unwrap().to_owned()
                        }
                    })
                )
            }
            2 => low_numbers(raw_number).unwrap_or_else(|| {
                let ones_value = raw_number % 10;
                let tens_string = tens(dbg!(10 * (raw_number / 10))).unwrap();
                if ones_value != 0 {
                    format!("{}-{}", tens_string, ones(raw_number % 10).unwrap())
                } else {
                    tens(10 * (raw_number / 10)).unwrap().to_owned()
                }
            }),
            1 => {
                format!("{}", ones(raw_number).unwrap())
            }
            _ => unreachable!(),
        };

        s.push_str(&number);

        if !group.is_empty() {
            s.push_str(&format!(" {}", group));
        }
        result.push(s);
    }
    result.into_iter().rev().collect::<Vec<_>>().join(" ")
}
