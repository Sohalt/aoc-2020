#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

struct ProgramState {
    acc: i32,
    pc: i32
}

fn parse_instruction(instruction: &str) -> Instruction {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+) ([+-][0-9]+)$").unwrap();
    }
    let caps = RE.captures(instruction).unwrap();
    let val:i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    match caps.get(1).unwrap().as_str() {
        "jmp" => { Instruction::Jmp(val) },
        "acc" => { Instruction::Acc(val) },
        _ => { Instruction::Nop(val) },
    }
}

fn interpret(instructions: &Vec<Instruction>) -> Result<i32, ProgramState> {
    let mut visited: HashMap<i32, bool> = HashMap::new();
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    loop {
        if pc == instructions.len() as i32 {
            return Ok(acc);
        }
        if *visited.get(&pc).unwrap_or(&false) {
            return Err(ProgramState { acc, pc });
        }
        visited.insert(pc, true);
        match instructions[pc as usize] {
            Instruction::Nop(_) => { pc += 1; },
            Instruction::Jmp(offset) => { pc += offset; },
            Instruction::Acc(val) => { pc += 1; acc += val; }
        }
    }
}

fn fix(mut instructions: Vec<Instruction>, offset: usize) -> Vec<Instruction> {
    let mut instruction = instructions.get_mut(offset).unwrap();
    match instruction {
        Instruction::Nop(val) => *instruction = Instruction::Jmp(*val),
        Instruction::Jmp(val) => *instruction = Instruction::Nop(*val),
        _ => ()
    };
    instructions
}

fn solve(instructions: Vec<Instruction>) -> i32{
    let mut variations = std::iter::successors(Some(1), |x| Some(x+1)).take(instructions.len()).map(|i| fix(instructions.clone(), i));
    variations.find_map(|instructions| interpret(&instructions).ok()).unwrap()
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = advent::get_input(8).await?;
    let instructions:Vec<Instruction> = input.lines().map(parse_instruction).collect();
    if let Err(p) = interpret(&instructions) {
    println!("{:?}", p.acc);
    println!("{:?}", solve(instructions));
    }
    Ok(())
}
