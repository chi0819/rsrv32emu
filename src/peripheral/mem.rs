use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::vec::Vec;

use crate::riscv::*;
use crate::utils::disassemble;

pub struct RAM {
    ram: Vec<u8>,
}

impl RAM {
    pub fn new() -> Self {
        Self {
            ram: vec![0u8; RAM_SIZE],
        }
    }

    pub fn load_word(&self, address: &usize) -> u32 {
        let bytes: [u8; 4] = self.ram.as_slice()[*address..=*address + 3]
            .try_into()
            .expect("Read word error");

        u32::from_le_bytes(bytes)
    }

    pub fn load_half(&self, address: &usize) -> u16 {
        let bytes: [u8; 2] = self.ram.as_slice()[*address..=*address + 1]
            .try_into()
            .expect("Read half error");

        u16::from_le_bytes(bytes)
    }

    pub fn load_byte(&self, address: &usize) -> u8 {
        *self.ram.get(*address).unwrap()
    }

    pub fn store_word(&mut self, address: &usize, value: &u32) {
        let bytes = value.clone().to_le_bytes();
        self.ram[*address..=*address + 3].copy_from_slice(&bytes);
    }

    pub fn store_half(&mut self, address: &usize, value: &u16) {
        let bytes = value.clone().to_le_bytes();
        self.ram[*address..=*address + 1].copy_from_slice(&bytes);
    }

    pub fn store_byte(&mut self, address: &usize, value: &u8) {
        self.ram[*address] = value.clone();
    }

    /* Load Binary to Memory */
    pub fn loader(&mut self, file_path: &str) {
        let path = Path::new(file_path);
        let mut file = File::open(path).unwrap();
        let _ = file.read(&mut self.ram);
    }

    /* Debug Usage Functions:
     * show_mem: Display the memory layout after load the program to memory
     */

    pub fn show_mem(&self) {
        println!("Show Disassemble RISC-V Instruction");
        let mut address: u32 = 0x0000_0000;
        for chunk in self.ram.chunks(4) {
            if chunk.len() == 4 {
                let instruction = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                let instruction_d = disassemble(instruction).unwrap();
                let disassemble_instruction: String =
                    match InstructionOpcode::from_u32(instruction_d.opcode.clone()).unwrap() {
                        InstructionOpcode::I => format!(
                            "{} x{}, x{}, {}",
                            InstructionTypeI::from_u32(instruction_d.funct3.clone().unwrap())
                                .unwrap()
                                .to_string()
                                .to_lowercase(),
                            instruction_d.rd.unwrap(),
                            instruction_d.rs1.unwrap(),
                            instruction_d.imm.unwrap()
                        ),
                        InstructionOpcode::L => format!(
                            "{} x{}, x{}, {}",
                            InstructionTypeL::from_u32(instruction_d.funct3.clone().unwrap())
                                .unwrap()
                                .to_string()
                                .to_lowercase(),
                            instruction_d.rd.unwrap(),
                            instruction_d.rs1.unwrap(),
                            instruction_d.imm.unwrap()
                        ),
                        InstructionOpcode::S => format!(
                            "{} x{}, x{}, {}",
                            InstructionTypeS::from_u32(instruction_d.funct3.clone().unwrap())
                                .unwrap()
                                .to_string()
                                .to_lowercase(),
                            instruction_d.rs2.unwrap(),
                            instruction_d.rs1.unwrap(),
                            instruction_d.imm.unwrap()
                        ),
                        InstructionOpcode::R => format!(
                            "{} x{}, x{}, x{}",
                            InstructionTypeR::from_u32(instruction_d.funct3.clone().unwrap())
                                .unwrap()
                                .to_string()
                                .to_lowercase(),
                            instruction_d.rd.unwrap(),
                            instruction_d.rs1.unwrap(),
                            instruction_d.rs2.unwrap()
                        ),
                        InstructionOpcode::B => format!(
                            "{} x{}, x{}, {:x}",
                            InstructionTypeB::from_u32(instruction_d.funct3.clone().unwrap())
                                .unwrap()
                                .to_string()
                                .to_lowercase(),
                            instruction_d.rs1.unwrap(),
                            instruction_d.rs2.unwrap(),
                            instruction_d.imm.unwrap() + address
                        ),
                        InstructionOpcode::JAL => format!(
                            "jal x{}, {}",
                            instruction_d.rd.unwrap(),
                            ((instruction_d.imm.unwrap() as i32) << 11) >> 11
                        ),
                        InstructionOpcode::JALR => format!(
                            "jalr x{}, x{}, {}",
                            instruction_d.rd.unwrap(),
                            instruction_d.rs1.unwrap(),
                            instruction_d.imm.unwrap()
                        ),
                        InstructionOpcode::LUI => format!(
                            "lui x{}, {}",
                            instruction_d.rd.unwrap(),
                            instruction_d.imm.unwrap()
                        ),
                        InstructionOpcode::AUIPC => format!(
                            "lui x{}, {}",
                            instruction_d.rd.unwrap(),
                            instruction_d.imm.unwrap()
                        ),
                        _ => String::from("NOT INSTRUCTION"),
                    };
                println!(
                    "0x{:04x}: 0x{:08x} {}",
                    &address, &instruction, &disassemble_instruction
                );
            } else {
                eprintln!("Warning: Incomplete instruction at the end of the RAM");
            }
            address += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const BASE_ADDRESS: usize = 0x0000;

    #[test]
    fn test_load_store_word() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut ram = RAM::new();
            let word = rng.gen_range(u32::MIN..u32::MAX);
            let address: usize = rng.gen_range(BASE_ADDRESS..RAM_SIZE - 4);
            ram.store_word(&address, &word);
            assert_eq!(&word, &ram.load_word(&address));
        }
    }

    #[test]
    fn test_load_store_half() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut ram = RAM::new();
            let half: u16 = rng.gen_range(u16::MIN..u16::MAX);
            let address: usize = rng.gen_range(BASE_ADDRESS..RAM_SIZE - 2);
            ram.store_half(&address, &half);
            assert_eq!(&half, &ram.load_half(&address));
        }
    }

    #[test]
    fn test_load_store_byte() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut ram = RAM::new();
            let byte: u8 = rng.gen_range(u8::MIN..u8::MAX);
            let address: usize = rng.gen_range(BASE_ADDRESS..RAM_SIZE - 1);
            ram.store_byte(&address, &byte);
            assert_eq!(&byte, &ram.load_byte(&address));
        }
    }
}
