fn solve(numbers: &Vec<u32>, target_sum: u32) -> Option<u32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r {
        if numbers[l] + numbers[r] > target_sum {
            r-=1;
        } else if numbers[l] + numbers[r] < target_sum {
            l +=1;
        } else if numbers[l] + numbers[r] == target_sum {
            return Some(numbers[l] * numbers[r])
        }
    }
    None
}

fn solve2(numbers: &Vec<u32>, target_sum: u32) -> Option<u32> {
    for n in numbers.iter() {
        if let Some(x) = solve(numbers, target_sum - n) {
            return Some(n * x)
        }
    }
    None
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let target_sum = 2020;
    let mut input = advent::get_input(1).await?.lines().map(|l| l.parse::<u32>().unwrap()).filter(|x| x < &target_sum).collect::<Vec<u32>>();
    input.sort();
    let answer = solve(&input, target_sum).unwrap();
    println!("{}", answer);
    let answer = solve2(&input, target_sum).unwrap();
    println!("{}", answer);
    Ok(())
}
