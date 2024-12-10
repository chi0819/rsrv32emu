use strum_macros::Display;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

/*
 * CPU Architecture: RISC-V 32 Bits
 */

pub const RAM_SIZE: usize = 1024; /* 1 KiB Memory */
pub const ADDR_WIDTH: u32 = 32;
pub const WORD_SIZE: u32 = 32;

/* Opcode 7 Bits */
#[macro_export]
macro_rules! opcode {
    ($instruction:expr) => {
        (($instruction >> 0) & 0x7F)
    };
}

/* Register Destination 5 Bits */
#[macro_export]
macro_rules! rd {
    ($instruction:expr) => {
        (($instruction >> 7) & 0x1F).try_into().unwrap()
    };
}

/* Register Source1 5 Bits */
#[macro_export]
macro_rules! rs1 {
    ($instruction:expr) => {
        (($instruction >> 15) & 0x1F).try_into().unwrap()
    };
}

/* Register Source2 5 Bits */
#[macro_export]
macro_rules! rs2 {
    ($instruction:expr) => {
        (($instruction >> 20) & 0x1F).try_into().unwrap()
    };
}

/* Funct3 3 Bits */
#[macro_export]
macro_rules! funct3 {
    ($instruction:expr) => {
        (($instruction >> 12) & 0x7)
    };
}

/* Funct7 7 Bits */
#[macro_export]
macro_rules! funct7 {
    ($instruction:expr) => {
        (($instruction >> 25) & 0x7F)
    };
}

/* Generate Immediate for U-Type Instruction */
#[macro_export]
macro_rules! immU {
    ($instruction:expr) => {
        ($instruction  & 0xFFFFF000)
    };
}

/* Generate Immediate for I-Type Instruction */
#[macro_export]
macro_rules! immI {
    ($instruction:expr) => {
        (($instruction >> 20) & 0xFFF)
    };
}

/* Generate Immediate for S-Type Instruction */
#[macro_export]
macro_rules! immS {
    ($instruction:expr) => {
        ((($instruction >> 20) & ($instruction >> 7)) & 0xFFF)
    };
}

/* Generate Immediate for B-Type Instruction */
#[macro_export]
macro_rules! immB {
    ($instruction:expr) => {{
        let imm_12 = (($instruction >> 31) & 0x1) << 12;
        let imm_10_5 = (($instruction >> 25) & 0x3F) << 5;
        let imm_4_1 = (($instruction >> 8) & 0xF) << 1;
        let imm_11 = (($instruction >> 7) & 0x1) << 11;

        imm_12 | imm_11 | imm_10_5 | imm_4_1
    }};
}

/* Generate Immediate for J-Type Instruction */
#[macro_export]
macro_rules! immJ {
    ($instruction:expr) => {{
        let imm_20 = (($instruction >> 31) & 0x1) << 20;
        let imm_10_1 = (($instruction >> 21) & 0x3FF) << 1;
        let imm_11 = (($instruction >> 20) & 0x1) << 11;
        let imm_19_12 = (($instruction >> 12) & 0xFF) << 12;

        imm_20 | imm_19_12 | imm_11 | imm_10_1
    }};
}

#[derive(Default)]
pub struct Instruction {
    pub opcode: u32,
    pub rs1: Option<usize>,
    pub rs2: Option<usize>,
    pub rd: Option<usize>,
    pub funct3: Option<u32>,
    pub funct7: Option<u32>,
    pub imm: Option<u32>
}

#[derive(Debug, Display)]
pub enum Mode {
    SHOWINSTRUCTION,
    RUN
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Display)]
pub enum InstructionOpcode {
    I     = 0b0010011,
    S     = 0b0100011,
    R     = 0b0110011,
    B     = 0b1100011,
    JAL   = 0b1101111,
    LUI   = 0b0110111,
    AUIPC = 0b0010111,
    CSR   = 0b1110011
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Display)]
pub enum InsturcitonTypeL {
    LB  = 0b000,
    LH  = 0b001,
    LW  = 0b010,
    LBU = 0b100,
    LHU = 0b101
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Display)]
pub enum InstructionsTypeI {
    ADDI  = 0b000,
    SLLI  = 0b001,
    SLTI  = 0b010,
    SLTIU = 0b011,
    XORI  = 0b100,
    SRI   = 0b101,
    ORI   = 0b110,
    ANDI  = 0b111
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Display)]
pub enum InstructionsTypeS {
    SB = 0b000,
    SH = 0b001,
    SW = 0b010
}

#[repr(u8)]
#[derive(FromPrimitive, Debug, Display)]
pub enum InstructionsTypeR {
    ADDSUB  = 0b000,
    SLL      = 0b001,
    SLT      = 0b010,
    SLTU     = 0b011,
    XOR      = 0b100,
    SR       = 0b101,
    OR       = 0b110,
    AND      = 0b111
}

#[repr(u8)]
#[derive(FromPrimitive, Debug, Display)]
pub enum InstructionsTypeB {
    BEQ  = 0b000,
    BNE  = 0b001,
    BLT  = 0b100,
    BGE  = 0b101,
    BLTU = 0b110,
    BGEU = 0b111
}
