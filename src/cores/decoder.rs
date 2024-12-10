use crate::*;
use crate::riscv::*;
use crate::cpu::*;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

pub fn decoder(cpu: &mut CPU) -> Result<Instruction, io::Error> {
    let instruction = cpu.fetch();
    let mut decoder_cache = Instruction::default();
    decoder_cache.opcode = opcode!(&instruction);
    let instructionType: InstructionOpcode = InstructionOpcode::from_u32(decoder_cache.opcode.clone())
                                                                .expect("Opcode Lookup Error");
    
    decoder_cache.imm = match instructionType {
        InstructionOpcode::I => Some(immI!(&instruction)),
        InstructionOpcode::S => Some(immS!(&instruction)),
        InstructionOpcode::B => Some(immB!(&instruction)),
        InstructionOpcode::JAL => Some(immJ!(&instruction)),
        InstructionOpcode::LUI | InstructionOpcode::AUIPC => Some(immU!(&instruction)),
        _ => None
    };

    decoder_cache.funct3 = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::I
        | InstructionOpcode::S
        | InstructionOpcode::B => Some(funct3!(&instruction)),
        _ => None
    };

    decoder_cache.funct7 = match instructionType {
        InstructionOpcode::R => Some(funct7!(&instruction)),
        _ => None
    };

    decoder_cache.rs1 = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::I
        | InstructionOpcode::S
        | InstructionOpcode::B => Some(rs1!(&instruction)),
        _ => None
    };

    decoder_cache.rs2 = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::S
        | InstructionOpcode::B => Some(rs1!(&instruction)),
        _ => Some(0)
    };

    decoder_cache.rd = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::I
        | InstructionOpcode::LUI
        | InstructionOpcode::AUIPC
        | InstructionOpcode::JAL => Some(rd!(&instruction)),
        _ => None
    };

    Ok(decoder_cache)
}
