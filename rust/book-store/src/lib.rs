use std::collections::HashSet;

const BOOK_PRICE: f64 = 800.0;

fn discount(nbooks: usize) -> u32 {
    (BOOK_PRICE
        * nbooks as f64
        * (1.0
            - match nbooks {
                1 => 0.0,
                2 => 0.05,
                3 => 0.10,
                4 => 0.20,
                5 => 0.25,
                _ => unreachable!(),
            })) as u32
}

// return value is in unit of US cents
pub fn lowest_price(basket: &[u32]) -> u32 {
    let mut groupings = vec![];
    let mut basket = basket.to_owned();

    // loop until the basket has been processed
    while !basket.is_empty() {
        // compute the unique book numbers in the basket
        let group = basket.clone().into_iter().collect::<HashSet<_>>();

        // we have a group of length `group.len()`
        groupings.push(group.len());

        // for each unique book
        for &book in group.iter() {
            // remove the first occurrence of that book
            if let Some(pos) = basket.iter().position(|&cur| cur == book) {
                basket.remove(pos);
            }
        }
    }

    // replace all three and five groupings with 4/4, since 4/4 is strictly better
    while groupings.contains(&3) && groupings.contains(&5) {
        if let Some(pos) = groupings.iter().position(|&x| x == 3) {
            groupings.remove(pos);
        }

        if let Some(pos) = groupings.iter().position(|&x| x == 5) {
            groupings.remove(pos);
        }

        groupings.extend_from_slice(&[4, 4]);
    }

    groupings.iter().map(|&g| discount(g)).sum()
}
