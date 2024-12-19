use crate::riscv::*;
use crate::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub fn disassemble(instruction: u32) -> Result<Instruction, io::Error> {
    let mut instruction_d = Instruction::default();
    instruction_d.opcode = opcode!(&instruction);
    println!("opcode: {:b}", &instruction_d.opcode);
    let instructionType: InstructionOpcode =
        InstructionOpcode::from_u32(instruction_d.opcode.clone()).expect("Opcode Lookup Error");

    instruction_d.imm = match instructionType {
        InstructionOpcode::I => Some(immI!(&instruction)),
        InstructionOpcode::L => Some(immL!(&instruction)),
        InstructionOpcode::S => Some(immS!(&instruction)),
        InstructionOpcode::B => Some(immB!(&instruction)),
        InstructionOpcode::JAL => Some(immJ!(&instruction)),
        InstructionOpcode::JALR => Some(immI!(&instruction)), /* Take care that JALR is I-Type Encoding */
        InstructionOpcode::LUI | InstructionOpcode::AUIPC => Some(immU!(&instruction)),
        _ => None,
    };

    instruction_d.funct3 = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::I
        | InstructionOpcode::L
        | InstructionOpcode::S
        | InstructionOpcode::B
        | InstructionOpcode::JALR => Some(funct3!(&instruction)),
        _ => None,
    };

    instruction_d.funct7 = match instructionType {
        InstructionOpcode::R => Some(funct7!(&instruction)),
        _ => None,
    };

    instruction_d.rs1 = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::I
        | InstructionOpcode::L
        | InstructionOpcode::S
        | InstructionOpcode::B
        | InstructionOpcode::JALR => Some(rs1!(&instruction)),
        _ => None,
    };

    instruction_d.rs2 = match instructionType {
        InstructionOpcode::R | InstructionOpcode::S | InstructionOpcode::B => {
            Some(rs2!(&instruction))
        }
        _ => None,
    };

    instruction_d.rd = match instructionType {
        InstructionOpcode::R
        | InstructionOpcode::I
        | InstructionOpcode::L
        | InstructionOpcode::LUI
        | InstructionOpcode::AUIPC
        | InstructionOpcode::JAL
        | InstructionOpcode::JALR => Some(rd!(&instruction)),
        _ => None,
    };

    Ok(instruction_d)
}
