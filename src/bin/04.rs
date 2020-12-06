#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Passport<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: Option<&'a str>,
}

fn parse_field(s: &str) -> Option<(&str, &str)> {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"([a-z]{3}):(.+)").unwrap();
    }
    let caps = RE.captures(s)?;
    Some((caps.get(1)?.as_str(), caps.get(2)?.as_str()))
}

fn parse_passport(s: &str) -> Option<Passport> {
    let fields = s.split(char::is_whitespace).collect::<Vec<&str>>();
    let mut map = HashMap::with_capacity(fields.len());
    fields.iter().for_each(|f| { if let Some((k, v)) = parse_field(f) { map.insert(k,v); } });
    Some(Passport {
        byr: map.get("byr")?,
        iyr: map.get("iyr")?,
        eyr: map.get("eyr")?,
        hgt: map.get("hgt")?,
        hcl: map.get("hcl")?,
        ecl: map.get("ecl")?,
        pid: map.get("pid")?,
        cid: map.get("cid").and_then(|x| Some(*x)),
    })
}

fn valid_byr(byr: &str) -> bool {
    byr.parse::<u32>().map_or(false, |byr| 1920 <= byr && byr <= 2002)
}

fn valid_iyr(iyr: &str) -> bool {
    iyr.parse::<u32>().map_or(false, |iyr| 2010 <= iyr && iyr <= 2020)
}

fn valid_eyr(eyr: &str) -> bool {
    eyr.parse::<u32>().map_or(false, |eyr| 2020 <= eyr && eyr <= 2030)
}

fn valid_hgt(hgt: &str) -> bool {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"([0-9]+)(cm|in)").unwrap();
    }
    if let Some(caps) = RE.captures(hgt) {
        let hgt = caps.get(1).and_then(|h| h.as_str().parse::<u32>().ok()).unwrap_or_default();
        match caps.get(2).and_then(|h| Some(h.as_str())).unwrap_or_default() {
            "cm" => 150 <= hgt && hgt <= 193,
            "in" => 59 <= hgt && hgt <= 76,
            _ => false
        }
    } else {
        false
    }
}

fn valid_hcl(hcl: &str) -> bool {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(hcl)
}

fn valid_ecl(ecl: &str) -> bool {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    }
    RE.is_match(ecl)
}

fn valid_pid(pid: &str) -> bool {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(pid)
}

fn validate_passport(passport: &Passport) -> bool {
  valid_byr(passport.byr) &&
  valid_iyr(passport.iyr) &&
  valid_eyr(passport.eyr) &&
  valid_hgt(passport.hgt) &&
  valid_hcl(passport.hcl) &&
  valid_ecl(passport.ecl) &&
  valid_pid(passport.pid)
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = advent::get_input(4).await?;
    let passports = input.split("\n\n").map(parse_passport).filter(Option::is_some).collect::<Vec<_>>();
    let count = passports.len();
    println!("{}", count);
    let count = passports.into_iter().map(Option::unwrap).filter(validate_passport).count();
    println!("{}", count);
    Ok(())
}
