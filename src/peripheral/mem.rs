use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use std::vec::Vec;
use crate::riscv::*;

pub struct RAM {
    ram: Vec<u8>
}

impl RAM {
    pub fn new() -> Self {
        Self {
            ram: vec![0u8; RAM_SIZE]
        }
    }

    pub fn read_word(&self, address: u32) -> u32 {
        10
    }

    pub fn write_word(&mut self, address: u32, data: u32) {

    }

    pub fn loader(&mut self, file_path: &str) {
        let path = Path::new(file_path);
        let mut file = File::open(path).unwrap();
        let _ = file.read(&mut self.ram);
    }

    /* Debug Usage Functions:
     */

    pub fn show_mem(&self) {
        println!("RISC-V Instruction");
        let mut address: u32 = 0x0000_0000;
        for chunk in self.ram.chunks(4) {
            if chunk.len() == 4 {
                let instruction = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                println!("0x{:08x}: 0x{:08x}", address, instruction);
            } else {
                eprintln!("Warning: Incomplete instruction at the end of the RAM");
            }
            address += 4;
        }
    }
}
