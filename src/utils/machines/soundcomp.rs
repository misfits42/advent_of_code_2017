use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum SoundComputerInstruction {
    Snd { value_1: String },
    Set { value_1: String, value_2: String },
    Add { value_1: String, value_2: String },
    Mul { value_1: String, value_2: String },
    Mod { value_1: String, value_2: String },
    Rcv { value_1: String },
    Jgz { value_1: String, value_2: String },
}

pub struct SoundComputer {
    input_queue: VecDeque<i64>,
    output_queue: VecDeque<i64>,
    single_mode_freqs_played: VecDeque<i64>,
    recovered_freqs: VecDeque<i64>,
    prog_c: usize,
    halted: bool,
    instructions: Vec<SoundComputerInstruction>,
    registers: HashMap<char, i64>,
    snd_count: usize,
    rcv_count: usize,
    awaiting_input: bool
}

/// Checks if the given string is a register name
fn is_reg_name(input: &String) -> bool {
    return !input.parse::<i64>().is_ok();
}

impl SoundComputer {
    pub fn new(instructions: &Vec<SoundComputerInstruction>) -> Self {
        let reg_state = SoundComputer::gen_initial_reg_state(instructions);
        Self {
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
            single_mode_freqs_played: VecDeque::new(),
            recovered_freqs: VecDeque::new(),
            prog_c: 0,
            halted: false,
            instructions: instructions.to_vec(),
            registers: reg_state,
            snd_count: 0,
            rcv_count: 0,
            awaiting_input: false
        }
    }

    /// Updates the specified register to the given value. If the register was not previously
    /// present in the SoundComputer, it is added and initialised to the given value.
    pub fn update_register(&mut self, reg: char, value: i64) {
        self.registers.insert(reg, value);
    }

    /// Returns the last frequency that was recovered by the SoundComputer, when executing in
    /// single-mode.
    pub fn last_recovered_freq(&self) -> Option<&i64> {
        return self.recovered_freqs.back();
    }

    /// Evaluates the provided value, depending on if it is a register name (single lowercase letter),
    /// or a raw value.
    fn evaluate_instruction_args_one_value(&self, value_1: &String) -> (Option<i64>, Option<i64>) {
        let val_1 = {
            if is_reg_name(value_1) {
                *self.registers.get(&value_1.chars().next().unwrap()).unwrap()
            } else {
                value_1.parse::<i64>().unwrap()
            }
        };
        return (Some(val_1), None);
    }

    /// Evaluates the provided values, depending on if they are register names (single lowercase
    /// letter) or a raw value.
    fn evaluate_instruction_args_two_values(&self, value_1: &String, value_2: &String) -> (Option<i64>, Option<i64>) {
        let val_1 = {
            if is_reg_name(value_1) {
                *self.registers.get(&value_1.chars().next().unwrap()).unwrap()
            } else {
                value_1.parse::<i64>().unwrap()
            }
        };
        let val_2 = {
            if is_reg_name(value_2) {
                *self.registers.get(&value_2.chars().next().unwrap()).unwrap()
            } else {
                value_2.parse::<i64>().unwrap()
            }
        };
        return (Some(val_1), Some(val_2));
    }

    /// Evaluates the arguments for the given instruction.
    fn evaluate_instruction_args(&self, instruction: &SoundComputerInstruction) -> (Option<i64>, Option<i64>) {
        match instruction {
            SoundComputerInstruction::Snd{value_1} => {
                return self.evaluate_instruction_args_one_value(&value_1);
            },
            SoundComputerInstruction::Set{value_1, value_2} => {
                return self.evaluate_instruction_args_two_values(&value_1, &value_2);
            },
            SoundComputerInstruction::Add{value_1, value_2} => {
                return self.evaluate_instruction_args_two_values(&value_1, &value_2);
            },
            SoundComputerInstruction::Mul{value_1, value_2} => {
                return self.evaluate_instruction_args_two_values(&value_1, &value_2);
            },
            SoundComputerInstruction::Mod{value_1, value_2} => {
                return self.evaluate_instruction_args_two_values(&value_1, &value_2);
            },
            SoundComputerInstruction::Rcv{value_1} => {
                return self.evaluate_instruction_args_one_value(&value_1);
            },
            SoundComputerInstruction::Jgz{value_1, value_2} => {
                return self.evaluate_instruction_args_two_values(&value_1, &value_2);
            }
        }
    }

    /// Gets the number of times the SoundComputer has sent a value, for receipt by the other
    /// SoundComputer in the duet.
    /// 
    /// Applicable when SoundComputer executed in double-mode.
    pub fn snd_count(&self) -> usize {
        return self.snd_count;
    }

    /// Indicates if the SoundComputer has attempted to receive an input value, but its input queue
    /// was empty on last attempt.
    pub fn is_awaiting_input(&self) -> bool {
        return self.awaiting_input;
    }

    /// Pops the first value from the SoundComputer output queue and returns the result. None is
    /// returned if the output queue is empty.
    pub fn pop_output(&mut self) -> Option<i64> {
        return self.output_queue.pop_front();
    }

    /// Pushes the given input value to the end of the SoundComputer input queue.
    pub fn push_input(&mut self, input: i64) {
        self.input_queue.push_back(input);
        self.awaiting_input = false;
    }

    /// Executes a single instruction in the SoundComputer, in either single-mode or double-mode.
    fn execute_step(&mut self, double_mode: bool) {
        // Check if program counter is outside of instruction list, reaching a halting condition
        if self.prog_c >= self.instructions.len() {
            self.halted = true;
            return;
        }
        // Evaluate the arguments of the current instruction
        let instruction = &self.instructions[self.prog_c];
        let args = self.evaluate_instruction_args(instruction);
        // check the current instruction
        match instruction {
            SoundComputerInstruction::Snd{value_1:_} => {
                self.prog_c += 1;
                self.snd_count += 1;
                if double_mode {
                    self.output_queue.push_back(args.0.unwrap());
                } else {
                    self.single_mode_freqs_played.push_back(args.0.unwrap());
                }
            },
            SoundComputerInstruction::Set{value_1, value_2:_} => {
                self.prog_c += 1;
                self.registers.insert(value_1.chars().next().unwrap(), args.1.unwrap());
            },
            SoundComputerInstruction::Add{value_1, value_2:_} => {
                self.prog_c += 1;
                *self.registers.get_mut(&value_1.chars().next().unwrap()).unwrap() += args.1.unwrap();
            },
            SoundComputerInstruction::Mul{value_1, value_2:_} => {
                self.prog_c += 1;
                *self.registers.get_mut(&value_1.chars().next().unwrap()).unwrap() *= args.1.unwrap();
            },
            SoundComputerInstruction::Mod{value_1, value_2:_} => {
                self.prog_c += 1;
                let remainder = self.registers.get(&value_1.chars().next().unwrap()).unwrap() % args.1.unwrap();
                self.registers.insert(value_1.chars().next().unwrap(), remainder);
            },
            SoundComputerInstruction::Rcv{value_1} => {
                if double_mode { // double-mode execution option
                    // Stop if awaiting input
                    if self.awaiting_input || self.input_queue.is_empty() {
                        self.awaiting_input = true;
                        return;
                    }
                    self.prog_c += 1;
                    let input_value = self.input_queue.pop_front().unwrap();
                    let reg = value_1.chars().next().unwrap();
                    self.registers.insert(reg, input_value);
                } else { // single-mode execution option
                    self.prog_c += 1;
                    let check_val = args.0.unwrap();
                    if check_val != 0 {
                        let rcv_freq = self.single_mode_freqs_played.back();
                        if rcv_freq.is_some() {
                            self.recovered_freqs.push_back(*rcv_freq.unwrap());
                            self.rcv_count += 1;
                        }
                    }
                }
            },
            SoundComputerInstruction::Jgz{value_1:_, value_2:_} => {
                let check_val = args.0.unwrap();
                // Check if we apply the jump instruction
                if check_val > 0 {
                    let jump_val = args.1.unwrap();
                    // Check if negative jump would jump back off top of instructions
                    if jump_val < 0 && jump_val.abs() as usize > self.prog_c {
                        self.prog_c = 0;
                        self.halted = true;
                        return;
                    }
                    // Conduct the jump
                    if jump_val < 0 {
                        self.prog_c -= jump_val.abs() as usize;
                    } else {
                        self.prog_c += jump_val.abs() as usize;
                    }
                } else {
                    self.prog_c += 1;
                }
            }
        }
    }

    pub fn is_halted(&self) -> bool {
        return self.halted;
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn execute_single_mode(&mut self) {
        loop {
            // Check if we have reached the break condition for single mode execution
            if self.rcv_count >= 1 {
                return;
            }
            // Stop execution immediately if the SoundComputer has already halted.
            if self.halted { 
                return;
            }
            // Check if there is an output value
            if !self.output_queue.is_empty() {
                let output_val = self.output_queue.pop_front().unwrap();
                self.input_queue.push_back(output_val);
            }
            self.execute_step(false);
        }
    }

    pub fn execute_double_mode(&mut self) -> Option<i64> {
        // Check if the SoundComputer has already halted
        if self.halted {
            return None;
        }
        // Execute one step
        self.execute_step(true);
        // Check if there is an output value queued
        if self.output_queue.is_empty() {
            return None;
        } else {
            return self.output_queue.pop_front();
        }
    }

    /// Generates the initial state for the registers referenced in the given instructions.
    fn gen_initial_reg_state(instructions: &Vec<SoundComputerInstruction>) -> HashMap<char, i64> {
        let mut reg_state = HashMap::<char, i64>::new();
        for instruction in instructions {
            match instruction {
                SoundComputerInstruction::Snd{value_1} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                },
                SoundComputerInstruction::Set{value_1, value_2} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                    if !value_2.parse::<i64>().is_ok() {
                        reg_state.insert(value_2.chars().next().unwrap(), 0);
                    }
                },
                SoundComputerInstruction::Add{value_1, value_2} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                    if !value_2.parse::<i64>().is_ok() {
                        reg_state.insert(value_2.chars().next().unwrap(), 0);
                    }
                },
                SoundComputerInstruction::Mul{value_1, value_2} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                    if !value_2.parse::<i64>().is_ok() {
                        reg_state.insert(value_2.chars().next().unwrap(), 0);
                    }
                },
                SoundComputerInstruction::Mod{value_1, value_2} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                    if !value_2.parse::<i64>().is_ok() {
                        reg_state.insert(value_2.chars().next().unwrap(), 0);
                    }
                },
                SoundComputerInstruction::Rcv{value_1} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                },
                SoundComputerInstruction::Jgz{value_1, value_2} => {
                    if !value_1.parse::<i64>().is_ok() {
                        reg_state.insert(value_1.chars().next().unwrap(), 0);
                    }
                    if !value_2.parse::<i64>().is_ok() {
                        reg_state.insert(value_2.chars().next().unwrap(), 0);
                    }
                }
            };
        }
        return reg_state;
    }
}
