use std::collections::HashSet;

fn process1(answers: &&str) -> u32 {
    let mut a:HashSet<char> = HashSet::new();
    answers.lines().for_each(|l| l.chars().for_each(|c| {a.insert(c);}));
    a.len() as u32
}

fn process2(answers: &&str) -> u32 {
    let people = answers.lines().map(|l| l.chars().collect::<HashSet<char>>()).collect::<Vec<_>>();
    people[0].iter().filter(|answer| people.iter().all(|p| p.contains(answer))).count() as u32
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = advent::get_input(6).await?;
    let groups = input.split("\n\n").collect::<Vec<&str>>();
    let sum:u32 = groups.iter().map(process1).sum();
    println!("{}", sum);
    let sum:u32 = groups.iter().map(process2).sum();
    println!("{}", sum);
    Ok(())
}
