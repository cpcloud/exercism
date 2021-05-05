use itertools::{EitherOrBoth, Itertools};
use std::str::FromStr;

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

fn tens(n: u64) -> Option<&'static str> {
    Some(match n {
        10 => "ten",
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

fn simple(n: u64) -> String {
    let hundreds_digit = n / 100;
    let tens_digit = (n - 100 * hundreds_digit) / 10;
    let ones_digit = n % 10;

    let mut result = vec![];

    if hundreds_digit != 0 {
        result.push(format!("{} hundred", ones(hundreds_digit).unwrap()));
    }

    if let Some(value) = ones(n).or_else(|| teens(n)) {
        result.push(value.to_owned());
    } else {
        let mut s = String::new();

        if let Some(value) = teens(n - 100 * hundreds_digit) {
            s.push_str(value);
        } else {
            if tens_digit != 0 {
                s.push_str(tens(tens_digit * 10).unwrap());

                if ones_digit != 0 {
                    s.push('-');
                }
            }

            if ones_digit != 0 {
                s.push_str(ones(ones_digit).unwrap());
            }
        }

        if !s.is_empty() {
            result.push(s);
        }
    }

    result.join(" ")
}

pub fn split_thousands(mut chars: Vec<char>) -> Vec<u64> {
    chars.reverse();
    chars
        .chunks_mut(3)
        .map(|chunk| {
            chunk.reverse();
            let res = chunk.iter().skip_while(|&&c| c == '0').collect::<String>();
            if res.is_empty() {
                0
            } else {
                u64::from_str(&res).unwrap()
            }
        })
        .rev()
        .collect::<Vec<_>>()
}

const SCALES: [&str; 6] = [
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

    let s = n.to_string();
    let splits = split_thousands(s.chars().collect::<Vec<_>>());
    let num_groups = splits.len();

    splits
        .into_iter()
        .map(|num| {
            if num == 0 {
                // use None to indicate whether we should print the suffix
                None
            } else {
                Some(if let Some(value) = ones(num).or_else(|| teens(num)) {
                    value.to_owned()
                } else {
                    simple(num)
                })
            }
        })
        .zip_longest(
            SCALES[..num_groups - 1]
                .iter()
                .rev()
                .map(|&s| Some(s.to_owned())),
        )
        .filter_map(|pair| {
            match pair {
                // we have a number chunk as well as a suffix that we should use
                EitherOrBoth::Both(Some(text), Some(suffix)) => {
                    Some(format!("{} {}", text, suffix))
                }

                // the number chunk is all zeros, we don't want to keep it in the description
                EitherOrBoth::Both(None, Some(_)) | EitherOrBoth::Left(None) => None,

                // we always have a non-None suffix if zip_longest returns a value for the right side,
                // because there are n number chunks and n - 1 suffixes
                EitherOrBoth::Both(_, None) => panic!("suffix should never be None"),

                // No suffix happens on the final element of the iteration
                EitherOrBoth::Left(Some(text)) => Some(text),

                // The splits are guaranteed to be length 1 and at least one larger than the number of
                // zipped suffixes
                EitherOrBoth::Right(_) => panic!("suffix exists but number text doesn't"),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
