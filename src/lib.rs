use std::env;
use reqwest::header::{HeaderValue,COOKIE};

pub async fn get_input(day: i32) -> reqwest::Result<String> {
    let url = format!("https://adventofcode.com/2020/day/{day}/input", day=day);
    let session = env::var("AOC_SESSION").expect("Please set environment variable AOC_SESSION to your advent of code session cookie");
    let client = reqwest::Client::new();
    client.get(&url)
          .header(COOKIE, HeaderValue::from_bytes(format!("session={}", session).as_bytes()).unwrap())
          .send().await?.text().await
}
