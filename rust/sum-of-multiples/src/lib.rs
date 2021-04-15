fn is_multiple_of(number: u32, factors: &[u32]) -> bool {
    factors
        .iter()
        .filter(|&&factor| factor != 0)
        .any(|&factor| number % factor == 0)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&number| is_multiple_of(number, factors))
        .sum()
}
