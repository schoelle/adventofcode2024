use adventofcode2024::input::AocInput;
use std::time::Instant;

struct State {
    a:i64, b:i64, c:i64
}

fn combo(v: i64, s: &State) -> i64 {
    match v {
        0..=3 => v,
        4 => s.a,
        5 => s.b,
        6 => s.c,
        _ => panic!("Illeagal value in combo")
    }
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day17.txt");
    let reg_strings = input.read_lines();
    let reg_a = reg_strings[0].split(": ").last().unwrap().parse::<i64>().unwrap();
    let reg_b = reg_strings[1].split(": ").last().unwrap().parse::<i64>().unwrap();
    let reg_c = reg_strings[2].split(": ").last().unwrap().parse::<i64>().unwrap();
    let code_string = input.read_line();
    let code: Vec<i64> = code_string
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    let mut pc = 0;
    let mut out: Vec<String> = Vec::new();
    let mut state: State = State { a: reg_a, b: reg_b, c: reg_c };
    while pc < code.len() {
        let instr = code[pc];
        let operand = code[pc+1];
        match instr {
            0 => {
                state.a =  state.a >> combo(operand, &state);
            }
            1 => {
                state.b = state.b ^ operand;
            }
            2 => {
                state.b = combo(operand, &state).rem_euclid(8);
            }
            3 => {
                if state.a != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => {
                state.b = state.b ^ state.c;
            }
            5 => {
                out.push(combo(operand, &state).rem_euclid(8).to_string());
            }
            6 => {
                state.b =  state.a >> combo(operand, &state);
            }
            7 => {
                state.c =  state.a >> combo(operand, &state);
            }
            _ => panic!("Unknown opcode")
        }
        pc += 2;
    }
    println!("Part 1: {}", out.join(","));
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
