#![feature(map_first_last)]
use std::collections::BTreeSet;

fn find_invalid(numbers: &Vec<i64>, window_size: usize) -> i64 {
    let mut window: BTreeSet<i64> = numbers.iter().cloned().take(window_size).collect();
    let mut i = window_size;
    while i < numbers.len() {
        let num = numbers[i];
        let min_window = window.first().unwrap();
        if num < min_window * 2 + 1 {
            return num;
        }
        if !window.iter().any(|n| *n != num - *n && window.contains(&(num-*n))) {
            return num;
        }
        window.insert(num);
        window.remove(&numbers[i - window_size]);
        i += 1;
    }
    0
}

fn find_seq<'a>(numbers: &'a Vec<i64>, target: i64) -> &'a[i64] {
   let mut start = 0;
   let mut end = 0;
    let mut acc = numbers[0];
    while acc != target {
        if acc < target {
            end += 1;
            acc += numbers[end];
        } else {
            acc -= numbers[start];
            start += 1;
        }
    }
    &numbers[start..end]
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input: Vec<i64> = advent::get_input(9)
        .await?
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let invalid = find_invalid(&input, 25);
    println!("{}", invalid);
    let seq = find_seq(&input, invalid);
    let min = seq.iter().min().unwrap();
    let max = seq.iter().max().unwrap();
    println!("{}", min+max);
    Ok(())
}
