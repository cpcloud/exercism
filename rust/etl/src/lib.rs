use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&points, letters)| {
            letters
                .into_iter()
                .map(char::to_ascii_lowercase)
                .zip(std::iter::repeat(points))
        })
        .collect()
}
