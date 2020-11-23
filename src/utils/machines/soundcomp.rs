use std::collections::HashMap;

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
    last_freq_played: Option<i64>,
    recovered_freqs: Vec<i64>,
    prog_c: usize,
    halted: bool,
    instructions: Vec<SoundComputerInstruction>,
    registers: HashMap<char, i64>
}

/// Checks if the given string is a register name
fn is_reg_name(input: &String) -> bool {
    return !input.parse::<i64>().is_ok();
}

impl SoundComputer {
    pub fn new(instructions: &Vec<SoundComputerInstruction>) -> Self {
        let reg_state = SoundComputer::gen_initial_reg_state(instructions);
        Self {
            last_freq_played: None,
            recovered_freqs: vec![],
            prog_c: 0,
            halted: false,
            instructions: instructions.to_vec(),
            registers: reg_state
        }
    }

    pub fn last_freq_played(&self) -> Option<i64> {
        return self.last_freq_played;
    }

    pub fn last_recovered_freq(&self) -> Option<i64> {
        if !self.recovered_freqs.is_empty() {
            let i = self.recovered_freqs.len() - 1;
            return Some(self.recovered_freqs[i]);
        } else {
            return None;
        }
    }

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

    pub fn execute(&mut self, break_on_rcv: bool) {
        // Stop execution immediately if the SoundComputer has already halted.
        if self.halted { 
            return;
        }
        loop {
            // Check if program counter is outside of instruction list
            if self.prog_c >= self.instructions.len() {
                self.halted = true;
                return;
            }
            let instruction = &self.instructions[self.prog_c];
            let args = self.evaluate_instruction_args(instruction);
            // check the current instruction
            match instruction {
                SoundComputerInstruction::Snd{value_1:_} => {
                    self.prog_c += 1;
                    self.last_freq_played = Some(args.0.unwrap());
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
                SoundComputerInstruction::Rcv{value_1:_} => {
                    self.prog_c += 1;
                    let check_val = args.0.unwrap();
                    if check_val != 0 {
                        let rcv_freq = self.last_freq_played;
                        if rcv_freq.is_some() {
                            self.recovered_freqs.push(rcv_freq.unwrap());
                            if break_on_rcv {
                                return
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
