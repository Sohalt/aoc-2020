#[macro_use] extern crate lazy_static;
use regex::Regex;

fn valid1(password: &Password) -> bool {
    let mut count:u32 = 0;
    for c in password.password.chars() {
        if c == password.character {
            count += 1;
        }
        if count > password.max {
            return false;
        }
    }
    return count >= password.min
}

fn valid2(password: &Password) -> bool {
    let p = password.password;
    let c = password.character;
    let mut chars = p.chars();
    let a = chars.nth((password.min - 1) as usize).unwrap();
    let b = chars.nth((password.max - password.min - 1) as usize).unwrap();
    (a == c && b != c) || (a != c && b == c)
}

struct Password<'a> {
    min: u32,
    max: u32,
    character: char,
    password: &'a str,
}

fn parse<'a>(s:&'a str) -> Password<'a> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    }
    let caps = RE.captures(s).unwrap();
    Password {
        min: caps.get(1).unwrap().as_str().parse().unwrap(),
        max: caps.get(2).unwrap().as_str().parse().unwrap(),
        character: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        password: caps.get(4).unwrap().as_str(),
    }
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = advent::get_input(2).await?;
    let passwords = input.lines().map(parse).collect::<Vec<Password>>();
    let answer = &passwords.iter().map(valid1).filter(|x| *x).count();
    println!("{}", answer);
    let answer = &passwords.iter().map(valid2).filter(|x| *x).count();
    println!("{}", answer);
    Ok(())
}
