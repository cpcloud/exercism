enum Verse {
    Zero,
    One,
    Two,
    More(u32),
}

impl From<u32> for Verse {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            n => Self::More(n),
        }
    }
}

impl Verse {
    fn verse(&self) -> String {
        match self {
            Self::Zero => {
                vec![
                    "No more bottles of beer on the wall, no more bottles of beer.",
                    "Go to the store and buy some more, 99 bottles of beer on the wall.",
                ]
                .join("\n")
                    + "\n"
            }
            Self::One => {
                vec![
                    "1 bottle of beer on the wall, 1 bottle of beer.",
                    "Take it down and pass it around, no more bottles of beer on the wall.",
                ]
                .join("\n")
                    + "\n"
            }
            Self::Two => {
                vec![
                    "2 bottles of beer on the wall, 2 bottles of beer.",
                    "Take one down and pass it around, 1 bottle of beer on the wall.",
                ]
                .join("\n")
                    + "\n"
            }
            Self::More(n) => {
                vec![
                    format!("{0} bottles of beer on the wall, {0} bottles of beer.", n),
                    format!(
                        "Take one down and pass it around, {} bottles of beer on the wall.",
                        n - 1
                    ),
                ]
                .join("\n")
                    + "\n"
            }
        }
    }
}

pub fn verse(n: u32) -> String {
    Verse::from(n).verse()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
