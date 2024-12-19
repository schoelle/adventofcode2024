use adventofcode2024::input::AocInput;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    a: i64,
    b: i64,
    c: i64,
}

impl State {
    fn combo(&self, v: i64) -> i64 {
        match v {
            0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Illeagal value in combo"),
        }
    }
}

fn simulate(init: &State, code: &Vec<i64>) -> Vec<i64> {
    let mut pc = 0;
    let mut out: Vec<i64> = Vec::new();
    let mut state: State = init.clone();
    while pc < code.len() {
        let instr = code[pc];
        let operand = code[pc + 1];
        match instr {
            0 => {
                state.a = state.a >> state.combo(operand);
            }
            1 => {
                state.b = state.b ^ operand;
            }
            2 => {
                state.b = state.combo(operand).rem_euclid(8);
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
                out.push(state.combo(operand).rem_euclid(8));
            }
            6 => {
                state.b = state.a >> state.combo(operand);
            }
            7 => {
                state.c = state.a >> state.combo(operand);
            }
            _ => panic!("Unknown opcode"),
        }
        pc += 2;
    }
    out
}

fn solve(code: &Vec<i64>, base: i64) -> Vec<i64> {
    let mut res = Vec::new();
    let sol = simulate(
        &State {
            a: base,
            b: 0,
            c: 0,
        },
        code,
    );
    if code.ends_with(&sol) {
        if code.len() == sol.len() {
            res.push(base);
        } else {
            for i in 0..8 {
                let nb = (base << 3) + i;
                res.append(&mut solve(code, nb));
            }
            return res;
        }
    }
    res
}

fn part1(init: &State, code: &Vec<i64>) {
    let res = simulate(init, code);
    let list: Vec<String> = res.iter().map(|s| s.to_string()).collect();
    println!("Part 1: {}", list.join(","))
}

fn part2(code: &Vec<i64>) {
    let mut res: Vec<i64> = Vec::new();
    for base in 0..8 {
        let sol = solve(code, base);
        res.extend(&sol);
    }
    println!("Part 2: {:?}", res.iter().min().unwrap());
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day17.txt");
    let reg_strings = input.read_lines();
    let a = reg_strings[0]
        .split(": ")
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let b = reg_strings[1]
        .split(": ")
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let c = reg_strings[2]
        .split(": ")
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let code_string = input.read_line();
    let code: Vec<i64> = code_string
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    let state = State { a, b, c };
    part1(&state, &code);
    part2(&code);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
