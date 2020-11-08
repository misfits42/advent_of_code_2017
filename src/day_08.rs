use std::collections::HashMap;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Operation {
    Increment,
    Decrement
}

impl Operation {
    fn from_string(input: &str) -> Option<Operation> {
        match input {
            "inc" => return Some(Operation::Increment),
            "dec" => return Some(Operation::Decrement),
            _ => return None
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Condition {
    Gt, // Greater than (>)
    Lt, // Less than (<)
    Gte, // Greater than or equal to (>=)
    Eql, // Equal to (==)
    Lte, // Less than or equal to (<=)
    Neq // Not equal to (!=)
}

impl Condition {
    fn from_string(input: &str) -> Option<Condition> {
        match input {
            ">" => Some(Condition::Gt),
            "<" => Some(Condition::Lt),
            ">=" => Some(Condition::Gte),
            "==" => Some(Condition::Eql),
            "<=" => Some(Condition::Lte),
            "!=" => Some(Condition::Neq),
            _ => None
        }
    }
}

struct Instruction {
    target_reg: String,
    op: Operation,
    op_val: i64,
    check_reg: String,
    cond: Condition,
    check_val: i64
}

impl Instruction {
    pub fn new(target_reg: &str, op: Operation, op_val: i64, check_reg: &str, cond: Condition, check_val: i64) -> Self {
        Self {
            target_reg: target_reg.to_string(),
            op: op,
            op_val: op_val,
            check_reg: check_reg.to_string(),
            cond: cond,
            check_val: check_val
        }
    }
}

#[aoc_generator(day8)]
fn generate_input(input: &str) -> (HashMap<String, i64>, Vec<Instruction>) {
    // Initialise variables to hold initial register state and list of instructions
    let mut registers = HashMap::<String, i64>::new();
    let mut instructions = Vec::<Instruction>::new();
    // Create regex to perform matching and value extraction from input lines
    let instruct_regex = Regex::new(r"([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (.*) (-?\d+)").unwrap();
    // Process each line of input
    for line in input.lines() {
        let line = line.trim(); // Remove any leading or trailing whitespace
        // Match line to regex
        for capture in instruct_regex.captures_iter(line) {
            // Extract fields from capture match
            let target_reg = capture[1].to_string();
            let op = Operation::from_string(&capture[2]).unwrap();
            let op_val = capture[3].parse::<i64>().unwrap();
            let check_reg = capture[4].to_string();
            let cond = Condition::from_string(&capture[5]).unwrap();
            let check_val = capture[6].parse::<i64>().unwrap();
            //// println!("{}, {:?}, {}, {}, {:?}, {}", target_reg, op, op_val, check_reg, cond, check_val);
            // Create new instruction and add to list
            let instruction = Instruction::new(&target_reg, op, op_val, &check_reg, cond, check_val);
            instructions.push(instruction);
            // Initialise new register values if not previously seen
            if !registers.contains_key(&target_reg) {
                registers.insert(target_reg.to_string(), 0);
            }
            if !registers.contains_key(&check_reg) {
                registers.insert(check_reg.to_string(), 0);
            }
        }
    } 
    // Return initial register states and list of instructions to process
    return (registers, instructions);
}

#[aoc(day8, part1)]
fn solve_part_1(input: &(HashMap<String, i64>, Vec<Instruction>)) -> i64 {
    // Create a mutable copy of the register state
    let mut registers = input.0.clone();
    // Process each of the provided instructions
    for instruct in &input.1 {
        // Check instruction condition
        let check_reg_val = *registers.get(&instruct.check_reg).unwrap();
        let check_val = instruct.check_val;
        let cond = instruct.cond;
        let cond_valid = match cond {
            Condition::Gt => check_reg_val > check_val,
            Condition::Lt => check_reg_val < check_val,
            Condition::Gte => check_reg_val >= check_val,
            Condition::Eql => check_reg_val == check_val,
            Condition::Lte => check_reg_val <= check_val,
            Condition::Neq => check_reg_val != check_val
        };
        // If the condition is valid, perform the operation on the target register
        if cond_valid {
            let op = instruct.op;
            let op_val = instruct.op_val;
            let target_reg = instruct.target_reg.to_string();
            match op {
                Operation::Increment => {
                    *registers.get_mut(&target_reg).unwrap() += op_val;
                },
                Operation::Decrement => {
                    *registers.get_mut(&target_reg).unwrap() -= op_val;
                }
            }
        }
    }
    // Check final register state to find largest overall value
    let max_value = *registers.values().max().unwrap();
    return max_value;
}

#[aoc(day8, part2)]
fn solve_part_2(input: &(HashMap<String, i64>, Vec<Instruction>)) -> i64 {
    // Create a mutable copy of the register state
    let mut registers = input.0.clone();
    let mut max_overall = i64::MIN;
    // Process each of the provided instructions
    for instruct in &input.1 {
        // Check instruction condition
        let check_reg_val = *registers.get(&instruct.check_reg).unwrap();
        let check_val = instruct.check_val;
        let cond = instruct.cond;
        let cond_valid = match cond {
            Condition::Gt => check_reg_val > check_val,
            Condition::Lt => check_reg_val < check_val,
            Condition::Gte => check_reg_val >= check_val,
            Condition::Eql => check_reg_val == check_val,
            Condition::Lte => check_reg_val <= check_val,
            Condition::Neq => check_reg_val != check_val
        };
        // If the condition is valid, perform the operation on the target register
        if cond_valid {
            let op = instruct.op;
            let op_val = instruct.op_val;
            let target_reg = instruct.target_reg.to_string();
            match op {
                Operation::Increment => {
                    *registers.get_mut(&target_reg).unwrap() += op_val;
                },
                Operation::Decrement => {
                    *registers.get_mut(&target_reg).unwrap() -= op_val;
                }
            }
            // Check if resulting value is the highest register value seen yet
            let result_val = *registers.get(&target_reg).unwrap();
            if result_val > max_overall {
                max_overall = result_val;
            }
        }
    }
    return max_overall;
}
