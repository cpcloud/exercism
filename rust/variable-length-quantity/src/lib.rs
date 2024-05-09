#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const LAST_BYTE_MASK: u8 = 0b1111_1110;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = vec![];
    eprintln!("{:#b}", 129);

    for [first, second, third, fourth] in values.iter().copied().map(|value| value.to_le_bytes()) {
        dbg!([first, second, third, fourth]);
    }
    if result.is_empty() {
        result.push(0);
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}
