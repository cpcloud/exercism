//Annotate each square of the given minefield with the number of mines that surround said square
// (blank if there are no surrounding mines)
//
use std::cmp::Ordering;

fn add_offset(r: usize, offset: isize, max: usize) -> Option<usize> {
    match offset.cmp(&0) {
        Ordering::Greater => {
            let value = r + offset as usize;
            if value > max - 1 {
                None
            } else {
                Some(value)
            }
        }
        Ordering::Less => r.checked_sub(offset.abs() as usize),
        Ordering::Equal => Some(r),
    }
}

fn count_nearby(minefield: &[&str], r: usize, c: usize) -> usize {
    const ROW_OFFSETS: [isize; 3] = [-1, 0, 1];
    const COLUMN_OFFSETS: [isize; 3] = [-1, 0, 1];

    let nrows = minefield.len();
    let ncolumns = minefield[0].len();

    itertools::iproduct!(&ROW_OFFSETS, &COLUMN_OFFSETS)
        .filter(|(&r, &c)| r != 0 || c != 0)
        .fold(0, |total, (&row_offset, &column_offset)| {
            total
                + add_offset(r, row_offset, nrows)
                    .and_then(|r| {
                        add_offset(c, column_offset, ncolumns)
                            .map(|c| usize::from(&minefield[r][c..c + 1] == "*"))
                    })
                    .unwrap_or(0)
        })
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(r, &row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)| {
                    if ch != '*' {
                        match count_nearby(minefield, r, c) {
                            0 => ' ',
                            nearby_mines => char::from_digit(nearby_mines as u32, 10).unwrap(),
                        }
                    } else {
                        '*'
                    }
                })
                .collect()
        })
        .collect()
}
