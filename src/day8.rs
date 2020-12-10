#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Instruction {
    pub fn parse(input: String) -> Self {
        let mut parts = input.split_ascii_whitespace();
        match parts.next().unwrap() {
            "acc" => Self::ACC(parts.next().unwrap().parse().unwrap()),
            "jmp" => Self::JMP(parts.next().unwrap().parse().unwrap()),
            "nop" => Self::NOP(parts.next().unwrap().parse().unwrap()),
            _ => panic!("{}", input),
        }
    }
    pub fn mutate_nop_jmp(&self) -> Self {
        match self {
            Self::NOP(i) => Self::JMP(*i),
            Self::JMP(i) => Self::NOP(*i),
            Self::ACC(_) => *self,
        }
    }
}
pub type InstructionVisited = (Instruction, bool);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VM {
    pub instructions: Vec<InstructionVisited>,
    pub acc: i32,
    pub ptr: usize,
}

impl VM {
    pub fn parse(input: String) -> Self {
        Self {
            instructions: input
                .lines()
                .map(|line| (Instruction::parse(line.to_owned()), false))
                .collect(),
            acc: 0,
            ptr: 0,
        }
    }
    pub fn halt_at_loop(&mut self) -> i32 {
        let len = self.instructions.len();
        loop {
            if self.ptr >= len {
                panic!("Pointer too big: {} {}", self.ptr, len);
            }
            use Instruction::*;
            let instruction = &mut self.instructions[self.ptr];
            match instruction {
                (_, true) => return self.acc,
                (NOP(_), _) => self.ptr += 1,
                (ACC(amt), _) => {
                    self.acc += *amt;
                    self.ptr += 1;
                }
                (JMP(amt), _) => self.ptr = (self.ptr as i32 + *amt) as usize,
            }
            instruction.1 = true;
        }
    }
    pub fn ends_properly(&mut self) -> bool {
        let len = self.instructions.len();
        loop {
            if self.ptr == (len - 1) {
                println!("Ends Properly: ACC = {}", self.acc);
                return true;
            }
            use Instruction::*;
            let instruction = &mut self.instructions[self.ptr];
            match instruction {
                (_, true) => {
                    // println!("Terminated at {}: {:?}\tACC = {}", self.ptr, inst, self.acc);
                    return false;
                }
                (NOP(_), _) => self.ptr += 1,
                (ACC(amt), _) => {
                    self.acc += *amt;
                    self.ptr += 1;
                }
                (JMP(amt), _) => self.ptr = (self.ptr as i32 + *amt) as usize,
            }
            instruction.1 = true;
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.ptr = 0;
    }
}

#[aoc_generator(day8)]
pub fn parse_input(inp: &str) -> String {
    inp.to_owned()
}

#[aoc(day8, part1)]
pub fn solve_part1(inp: &String) -> i32 {
    let mut vm = VM::parse(inp.clone());
    vm.halt_at_loop()
}

#[aoc(day8, part2)]
pub fn solve_part2(inp: &String) -> i32 {
    let vm = VM::parse(inp.clone());
    let flipping_targets: Vec<_> = vm
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, (instruction, _))| {
            if let Instruction::ACC(_) = instruction {
                false
            } else {
                true
            }
        })
        .collect();
    let flipping_target_indices: Vec<_> = flipping_targets.iter().map(|(i, _)| i).collect();
    println!("{}", flipping_target_indices.len());
    for i in flipping_target_indices {
        //flip nop-jmp
        let mut vm = vm.clone();
        print!("{:?} => ", vm.instructions[*i].0);
        vm.instructions[*i].0 = vm.instructions[*i].0.mutate_nop_jmp();
        println!("{:?} ", vm.instructions[*i].0);
        if vm.ends_properly() {
            return vm.acc;
        }
    }
    panic!("None?")
}
