pub fn raindrops(n: u32) -> String {
    let mut result = vec![];

    if n % 3 == 0 {
        result.push("Pling".to_owned());
    }

    if n % 5 == 0 {
        result.push("Plang".to_owned());
    }

    if n % 7 == 0 {
        result.push("Plong".to_owned());
    }

    if result.is_empty() {
        result.push(n.to_string());
    }

    result.join("")
}
