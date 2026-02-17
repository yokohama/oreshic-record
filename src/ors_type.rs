use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum UnitType {
    MdFile,
    Section,
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitType::MdFile => write!(f, "MD"),
            UnitType::Section => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RecordType {
    Command,
    Track,
    Writeup,
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordType::Command => write!(f, "cmd"),
            RecordType::Track => write!(f, "trk"),
            RecordType::Writeup => write!(f, "wu"),
        }
    }
}
