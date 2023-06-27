#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    PDecrement,
    PIncrement,
    VDecrement,
    VIncrement,
    Out,
    In,
    LoopBegin,
    LoopEnd,
    NOOP
}

#[derive(Debug)]
pub struct UnsopportedSymbolError;

impl Symbol {
    pub fn from_character(c: char) -> Result<Self, UnsopportedSymbolError> {
        match c {
            '>' => Ok(Self::PIncrement),
            '<' => Ok(Self::PDecrement),
            '+' => Ok(Self::VIncrement),
            '-' => Ok(Self::VDecrement),
            '.' => Ok(Self::Out),
            ',' => Ok(Self::In),
            '[' => Ok(Self::LoopBegin),
            ']' => Ok(Self::LoopEnd),
            _ => Err(UnsopportedSymbolError),
        }
    }
}
