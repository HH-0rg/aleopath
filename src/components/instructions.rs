use crate::ByteCode;

use super::registers::Register;
use super::types::Literal;

#[derive(Debug)]
pub enum Opcode {
    Abs,
    AbsWrapped,
    Add,
    AddWrapped,
    And,
    AssertEq,
    AssertNeq,
    Call,
    Cast,
    CommitBHP256,
    CommitBHP512,
    CommitBHP768,
    CommitBHP1024,
    CommitPED64,
    CommitPED128,
    Div,
    DivWrapped,
    Double,
    GreaterThan,
    GreaterThanOrEqual,
    HashBHP256,
    HashBHP512,
    HashBHP768,
    HashBHP1024,
    HashPED64,
    HashPED128,
    HashPSD2,
    HashPSD4,
    HashPSD8,
    Inv,
    IsEq,
    IsNeq,
    LessThan,
    LessThanOrEqual,
    Mod,
    Mul,
    MulWrapped,
    Nand,
    Neg,
    Nor,
    Not,
    Or,
    Pow,
    PowWrapped,
    Rem,
    RemWrapped,
    Shl,
    ShlWrapped,
    Shr,
    ShrWrapped,
    Square,
    SquareRoot,
    Sub,
    SubWrapped,
    Ternary,
    Xor,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Abs,
            1 => Self::AbsWrapped,
            2 => Self::Add,
            3 => Self::AddWrapped,
            4 => Self::And,
            5 => Self::AssertEq,
            6 => Self::AssertNeq,
            7 => Self::Call,
            8 => Self::Cast,
            9 => Self::CommitBHP256,
            10 => Self::CommitBHP512,
            11 => Self::CommitBHP768,
            12 => Self::CommitBHP1024,
            13 => Self::CommitPED64,
            14 => Self::CommitPED128,
            15 => Self::Div,
            16 => Self::DivWrapped,
            17 => Self::Double,
            18 => Self::GreaterThan,
            19 => Self::GreaterThanOrEqual,
            20 => Self::HashBHP256,
            21 => Self::HashBHP512,
            22 => Self::HashBHP768,
            23 => Self::HashBHP1024,
            24 => Self::HashPED64,
            25 => Self::HashPED128,
            26 => Self::HashPSD2,
            27 => Self::HashPSD4,
            28 => Self::HashPSD8,
            29 => Self::Inv,
            30 => Self::IsEq,
            31 => Self::IsNeq,
            32 => Self::LessThan,
            33 => Self::LessThanOrEqual,
            34 => Self::Mod,
            35 => Self::Mul,
            36 => Self::MulWrapped,
            37 => Self::Nand,
            38 => Self::Neg,
            39 => Self::Nor,
            40 => Self::Not,
            41 => Self::Or,
            42 => Self::Pow,
            43 => Self::PowWrapped,
            44 => Self::Rem,
            45 => Self::RemWrapped,
            46 => Self::Shl,
            47 => Self::ShlWrapped,
            48 => Self::Shr,
            49 => Self::ShrWrapped,
            50 => Self::Square,
            51 => Self::SquareRoot,
            52 => Self::Sub,
            53 => Self::SubWrapped,
            54 => Self::Ternary,
            55 => Self::Xor,
            _ => unreachable!(),
        }
    }
}

const UNARY: &[Opcode] = &[
    Opcode::Abs,
    Opcode::AbsWrapped,
    Opcode::Double,
    Opcode::Inv,
    Opcode::Neg,
    Opcode::Not,
    Opcode::Square,
    Opcode::SquareRoot,
    Opcode::HashBHP256,
    Opcode::HashBHP512,
    Opcode::HashBHP768,
    Opcode::HashBHP1024,
    Opcode::HashPED64,
    Opcode::HashPED128,
    Opcode::HashPSD2,
    Opcode::HashPSD4,
    Opcode::HashPSD8,
];

const BINARY: &[Opcode] = &[
    Opcode::Add,
    Opcode::AddWrapped,
    Opcode::Sub,
    Opcode::SubWrapped,
    Opcode::Mul,
    Opcode::MulWrapped,
    Opcode::Div,
    Opcode::DivWrapped,
    Opcode::Rem,
    Opcode::RemWrapped,
    Opcode::Pow,
    Opcode::PowWrapped,
    Opcode::Shl,
    Opcode::ShlWrapped,
    Opcode::Shr,
    Opcode::ShrWrapped,
    Opcode::And,
    Opcode::Xor,
    Opcode::Or,
    Opcode::Nand,
    Opcode::Nor,
    Opcode::GreaterThan,
    Opcode::GreaterThanOrEqual,
    Opcode::LessThan,
    Opcode::LessThanOrEqual,
    Opcode::IsEq,
    Opcode::IsNeq,
    Opcode::CommitBHP256,
    Opcode::CommitBHP512,
    Opcode::CommitBHP768,
    Opcode::CommitBHP1024,
    Opcode::CommitPED64,
    Opcode::CommitPED128,
    Opcode::Mod,
];

const ASSERT: &[Opcode] = &[Opcode::AssertEq, Opcode::AssertNeq];
const IS_CHECK: &[Opcode] = &[Opcode::IsEq, Opcode::IsNeq];

#[derive(Debug)]
pub enum Operand {
    Literal(Literal),
    Register(Register),
    ProgramId(String),
    Caller(String),
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    operands: Option<Vec<Operand>>
}

impl Instruction {

    pub fn read(bytes: &mut ByteCode) -> Self {
        let opcode = Opcode::from(bytes.read_u16() as usize);
        match opcode {
            Opcode::Call => todo!(),
            _ => todo!()
        }
        todo!()
    }
}