#![feature(map_into_keys_values)]
use std::collections::{HashMap, HashSet};
use regex::Regex;
#[macro_use] extern crate lazy_static;

fn parse_bags(bags: &str) -> HashMap<&str, u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(([0-9]+) ([a-z]+ [a-z]+) bags?)").unwrap();
    }
    let mut m = HashMap::new();
    for c in RE.captures_iter(bags){
        let color = c.get(3).unwrap().as_str();
        let num = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
        m.insert(color, num);
    }
    m
}

fn parse_line(line: &str) -> (&str, HashMap<&str, u32>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]+ [a-z]+) bags contain (.+)\.").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let outer = caps.get(1).unwrap().as_str();
    let inner = parse_bags(caps.get(2).unwrap().as_str());
    (outer, inner)
}

fn parse(input: &str) -> Box<HashMap<&str,HashSet<&str>>> {
    let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let (outer, inner) = parse_line(l);
        inner.into_keys().for_each(|i| {
            m.entry(i).or_insert(HashSet::new()).insert(outer);
        });
    });
    Box::new(m)
}

fn parse2(input: &str) -> Box<HashMap<&str,HashMap<&str, u32>>> {
    let mut m: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    input.lines().for_each(|l| {
        let (outer, inner) = parse_line(l);
        m.insert(outer, inner);
    });
    Box::new(m)
}

fn part1(input: &str) -> u32 {
    let rules = parse(&input);
    let mut colors: HashSet<&str> = HashSet::new();
    let mut queue: Vec<&str> = vec!("shiny gold");
    while let Some(x) = queue.pop() {
        colors.insert(x);
        if let Some(outer) = rules.get(x) {
            outer.into_iter().for_each(|c| {if colors.insert(c) {queue.push(c)}})
        }
    }
    (colors.len()-1) as u32
}

fn part2(input: &str) -> u32 {
    let rules = parse2(&input);
    count_bags(&rules, "shiny gold") - 1
}

fn count_bags(rules: &Box<HashMap<&str, HashMap<&str, u32>>>, bag: &str) -> u32 {
    let inner = rules.get(bag).unwrap();
    inner.iter().map(|(b, n)| n * count_bags(&rules, b)).sum::<u32>() + 1
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = advent::get_input(7).await?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}
