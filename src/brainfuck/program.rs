use std::str::FromStr;

use super::symbol::Symbol;

#[derive(Debug)]
pub struct Program {
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
pub struct UnsupportedProgramError;

impl FromStr for Program {
    type Err = UnsupportedProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let characters = s.chars();
        let symbols = characters
            .map(|c| Symbol::from_character(c).unwrap_or(Symbol::NOOP))
            .filter(|s| *s != Symbol::NOOP)
            .collect::<Vec<_>>();
        Ok(Program { symbols })
    }
}

impl Program {
    pub fn get(&self, index: usize) -> Symbol {
        self.symbols[index]
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }
}
