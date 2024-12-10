/*
 * C Architecture: RISC-V 32 Bits
 */

pub const RAM_SIZE: usize = 1024; /* 1 KiB Memory */
pub const ADDR_WIDTH: u32 = 32;
pub const WORD_SIZE: u32 = 32;

#[repr(u8)]
enum InstructionOpcode {
    L     = 0b0000011,
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
enum InsturcitonTypeL {
    LB  = 0b000,
    LH  = 0b001,
    LW  = 0b010,
    LBU = 0b100,
    LHU = 0b101
}

#[repr(u8)]
enum InstructionsTypeI {
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
enum InstructionsTypeS {
    SB = 0b000,
    SH = 0b001,
    SW = 0b010
}

#[repr(u8)]
enum InstructionsTypeR {
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
enum InstructionsTypeB {
    BEQ  = 0b000,
    BNE  = 0b001,
    BLT  = 0b100,
    BGE  = 0b101,
    BLTU = 0b110,
    BGEU = 0b111
}
