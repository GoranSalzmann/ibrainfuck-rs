use std::{fs, str::FromStr};

use console::Term;

use super::{program::Program, symbol::Symbol};

const ARRAY_SIZE: usize = 30000;

#[derive(Debug)]
pub struct Interpreter {
    program: Option<Program>,
    pub cell_pointer: usize,
    pub program_counter: usize,
    cells: [u8; ARRAY_SIZE],
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            program: None,
            cell_pointer: 0,
            program_counter: 0,
            cells: [0; ARRAY_SIZE],
        }
    }

    #[allow(dead_code)]
    pub fn load_program_from_str(&mut self, program: &str) {
        self.program = Program::from_str(program).ok();
    }

    #[allow(dead_code)]
    pub fn load_program_from_file(&mut self, path: &str) {
        self.program = Program::from_str(fs::read_to_string(path).unwrap().as_str()).ok();
    }

    pub fn run(&mut self) {
        while self.has_operations() {
            let operation = self.program.as_ref().unwrap().get(self.program_counter);
            match operation {
                Symbol::PDecrement => {
                    self.cell_pointer = ((self.cell_pointer as isize) - 1)
                        .wrapping_rem_euclid(ARRAY_SIZE as isize)
                        as usize
                }
                Symbol::PIncrement => self.cell_pointer = (self.cell_pointer + 1) % ARRAY_SIZE,
                Symbol::VDecrement => {
                    self.cells[self.cell_pointer] = self.cells[self.cell_pointer].wrapping_sub(1)
                }
                Symbol::VIncrement => {
                    self.cells[self.cell_pointer] = self.cells[self.cell_pointer].wrapping_add(1)
                }
                Symbol::Out => print!("{}", self.cells[self.cell_pointer] as char),
                Symbol::In => {
                    let char = Term::read_char(&Term::stdout()).unwrap();
                    self.cells[self.cell_pointer] = char as u8;
                }
                Symbol::LoopBegin => {
                    if self.cells[self.cell_pointer] == 0 {
                        self.advance_pc();
                    }
                }
                Symbol::LoopEnd => {
                    if self.cells[self.cell_pointer] != 0 {
                        self.retreat_pc();
                    }
                }
                _ => {}
            }
            self.program_counter += 1;
        }
    }

    pub fn has_operations(&mut self) -> bool {
        if let Some(ref mut p) = self.program {
            self.program_counter < p.len()
        } else {
            return false;
        }
    }

    pub fn advance_pc(&mut self) {
        let mut opening_brackets = 0;
        let mut pos_found = false;
        while !pos_found {
            self.program_counter += 1;
            let next = self.program.as_ref().unwrap().get(self.program_counter);
            if next == Symbol::LoopBegin {
                opening_brackets += 1;
            } else if next == Symbol::LoopEnd {
                opening_brackets -= 1;
                if opening_brackets == -1 {
                    pos_found = true;
                }
            }
        }
    }

    pub fn retreat_pc(&mut self) {
        let mut closing_brackets = 0;
        let mut pos_found = false;
        while !pos_found {
            self.program_counter -= 1;
            let next = self.program.as_ref().unwrap().get(self.program_counter);
            if next == Symbol::LoopBegin {
                closing_brackets -= 1;
                if closing_brackets == -1 {
                    pos_found = true;
                }
            } else if next == Symbol::LoopEnd {
                closing_brackets += 1;
            }
        }
    }
}
