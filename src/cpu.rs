use crate::cores::decoder::*;
use crate::cores::registers::*;
use crate::peripheral::mem::*;
use crate::riscv::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub enum MSTATE {
    UNPRIVILEGE,
    PRIVILEGE,
    MACHINE,
}

impl MSTATE {
    pub fn show(&self) {
        match self {
            Self::UNPRIVILEGE => println!("Current is unprivilege mode"),
            Self::PRIVILEGE => println!("Current is privilege mode"),
            Self::MACHINE => println!("Current is machine mode"),
        }
    }
}

pub struct CPU {
    pc: usize,
    state: MSTATE,
    registers: RegFile,
    ram: RAM,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            state: MSTATE::UNPRIVILEGE,
            registers: RegFile::new(),
            ram: RAM::new(),
        }
    }

    pub fn run(&mut self) {
        let instruction: Instruction = decoder(self).unwrap();
        println!(
            "{} x{}, x{}, x{}",
            InstructionsTypeI::from_u32(instruction.funct3.clone().unwrap())
                .unwrap()
                .to_string()
                .to_lowercase(),
            &instruction.rd.unwrap(),
            &instruction.rs1.unwrap(),
            &instruction.imm.unwrap()
        )
    }

    pub fn mem_loader(&mut self, file_path: &str) {
        self.ram.loader(file_path);
    }

    pub fn fetch(&mut self) -> u32 {
        let instruction: u32 = self.ram.load_word(&self.pc);
        self.pc += 4;
        instruction
    }

    /* Debug Usage Functions:
     *
     */

    pub fn show_mem(&self) {
        self.ram.show_mem();
    }
}
