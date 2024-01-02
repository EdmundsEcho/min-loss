use std::collections::BTreeSet;
use std::convert::TryInto;
use std::io::{self, BufRead, Write};

fn find_minimum_loss(numbers: &[i64]) -> Option<i32> {
    let mut seen = BTreeSet::new();
    let mut min_loss: Option<i32> = None;

    for (i, &price) in numbers.iter().enumerate() {
        // Insert the current price along with its index into the set
        seen.insert((price, i));

        // Iterate over all elements in the set that are greater than the current price
        for &(seen_price, seen_index) in seen.range((price, i)..) {
            if seen_index < i {
                if let Ok(loss) = (seen_price - price).try_into() {
                    let loss: i32 = loss;
                    if loss < 0 {
                        continue;
                    }
                    min_loss = Some(min_loss.map_or(loss, |current_min| current_min.min(loss)));
                    if loss == 1 {
                        return min_loss; // Early return if loss is 1
                    }
                }
                break;
            }
        }
    }

    min_loss
}
fn minimum_loss(price: &[i64]) -> i32 {
    match find_minimum_loss(price) {
        Some(loss) => loss,
        None => panic!("No minimum loss found"),
    }
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _ = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let price: Vec<i64> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = minimum_loss(&price);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result).unwrap();
}
