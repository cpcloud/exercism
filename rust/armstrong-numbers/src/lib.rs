pub fn is_armstrong_number(num: u32) -> bool {
    let ndigits = f64::from(num).log10().floor() as u32 + 1;
    (0..ndigits)
        .rev()
        .scan(num, |n, digit| {
            let power = 10_u32.pow(digit);
            let multiple = *n / power;
            *n -= multiple * power;
            Some(multiple)
        })
        .map(|number| number.pow(ndigits))
        .sum::<u32>()
        == num
}
