use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Halve,
    Triple,
    Increment,
    Jump(i32),
    JumpIfEven(i32),
    JumpIfOdd(i32),
}

pub struct Command {
    pub inst: Instruction,
    pub register: Option<char>,
}

impl Command {
    pub fn new(inst: Instruction, maybe_reg: Option<&str>) -> Command {
        if let Some(reg) = maybe_reg {
            let next_char = reg.chars().next().unwrap();
            Command {
                inst,
                register: Some(next_char),
            }
        } else {
            Command {
                inst,
                register: None,
            }
        }
    }
}

pub fn parse_program(contents: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line
            .split(',')
            .flat_map(|x| x.split(' '))
            .filter(|s| s.len() > 0)
            .collect();

        match parts.as_slice() {
            ["hlf", reg] => commands.push(Command::new(Instruction::Halve, Some(reg))),
            ["tpl", reg] => commands.push(Command::new(Instruction::Triple, Some(reg))),
            ["inc", reg] => commands.push(Command::new(Instruction::Increment, Some(reg))),
            ["jmp", offset] => {
                commands.push(Command::new(Instruction::Jump(parse_offset(offset)), None))
            }
            ["jie", reg, offset] => commands.push(Command::new(
                Instruction::JumpIfEven(parse_offset(offset)),
                Some(reg),
            )),
            ["jio", reg, offset] => commands.push(Command::new(
                Instruction::JumpIfOdd(parse_offset(offset)),
                Some(reg),
            )),
            _ => panic!("Unparsable command: {:?}", parts),
        }
    }
    commands
}

pub fn parse_offset(s: &str) -> i32 {
    let mut raw_value = s.chars();
    let sign = raw_value.next().unwrap();
    let value = raw_value.collect::<String>().parse::<i32>().unwrap();
    if sign == '+' {
        value
    } else {
        -value
    }
}

pub fn evaluate_command(cmd: &Command, registers: &mut HashMap<char, i32>) -> i32 {
    let mut jump = 1;
    
    let reg_name: char = if let Some(reg) = cmd.register {reg} else {'?'};
    let reg_val: i32 = if let Some(val) = registers.get(&reg_name) {*val} else {0};

    match cmd.inst {
        Instruction::Increment => registers.insert(reg_name, reg_val + 1),
        Instruction::Halve => registers.insert(reg_name, reg_val / 2),
        Instruction::Triple => registers.insert(reg_name, reg_val * 3),
        Instruction::Jump(amt) => {
            jump = amt;
            None
        },
        Instruction::JumpIfEven(amt) => {
            jump = if reg_val % 2 == 0 {amt} else {1}; 
            None
        },
        Instruction::JumpIfOdd(amt) => {
            jump = if reg_val == 1 {amt} else {1}; 
            None
        },

    };

    jump
}

pub fn run(commands: &Vec<Command>, mut registers: HashMap<char, i32>) -> HashMap<char, i32> {
    let mut instruction = 0;
    let lower_bound = 0;
    let upper_bound = commands.len() as i32;

    while instruction >= lower_bound && instruction < upper_bound {
        let next_action = &commands[instruction as usize];
        // println!("On instruction: {} => {:?} ;; a = {}", instruction + 1, next_action.inst, registers.get(&'a').unwrap());
        let jump = evaluate_command(next_action, &mut registers);
        instruction += jump;
    }

    registers
}
