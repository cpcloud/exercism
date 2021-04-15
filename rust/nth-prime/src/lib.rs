fn is_prime(number: u32, seen_primes: &[u32]) -> bool {
    seen_primes
        .iter()
        .copied()
        .take_while(|&p| p < number)
        .all(|divisor| number % divisor != 0)
}

pub fn nth(target_index: u32) -> u32 {
    let mut number = 2;
    let mut current_index = 0;
    let mut seen_primes = vec![];

    loop {
        if is_prime(number, &seen_primes) {
            if current_index == target_index {
                return number;
            }
            seen_primes.push(number);
            current_index += 1;
        }
        number += 1;
    }
}
