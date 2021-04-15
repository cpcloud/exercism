use bit_set::BitSet;

fn is_prime(number: usize, seen_primes: &mut BitSet<usize>) -> bool {
    if seen_primes.contains(number) {
        true
    } else {
        seen_primes
            .iter()
            .take_while(|&p| (p as f64) < (number as f64).sqrt())
            .all(|divisor| number % divisor != 0)
    }
}

fn nth(target_index: usize, seen_primes: &mut BitSet<usize>) -> usize {
    let mut number = seen_primes.iter().last().unwrap_or(2);
    let mut current_index = seen_primes.len().checked_sub(1).unwrap_or(0);

    loop {
        if is_prime(number, seen_primes) {
            if current_index == target_index {
                return number;
            }
            seen_primes.insert(number as usize);
            current_index += 1;
        }
        number += 1;
    }
}

pub fn factors(mut n: usize) -> Vec<usize> {
    let mut factors = vec![];
    let mut i = 0;
    let mut seen_primes = BitSet::default();

    while n != 1 {
        let prime = nth(i, &mut seen_primes);

        loop {
            if n % prime == 0 {
                factors.push(prime);
                n /= prime;
            } else {
                break;
            }

            if n == 1 {
                break;
            }
        }

        i += 1;
    }

    factors
}
