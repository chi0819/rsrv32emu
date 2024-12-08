use crate::peripheral::mem::*;

pub enum MSTATE {
    UNPRIVILEGE,
    PRIVILEGE,
    MACHINE
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

pub struct RegFile {
   registers: [u32; 32]
}

impl RegFile {
    pub fn new() -> Self {
        Self {
            registers: [0; 32]
        }
    }

    pub fn read(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else {
            self.registers[index]
        }
    }

    pub fn write(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
    }

    /* Debug Usage Functions:
     *
     */
    pub fn show_regs(&self) {
        
    }
}

pub struct CPU {
    pc: u32,
    state: MSTATE,
    registers: RegFile,
    ram: RAM
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            state: MSTATE::UNPRIVILEGE,
            registers: RegFile::new(),
            ram: RAM::new()
        }
    }

    pub fn run(&mut self) {
        println!("Hello");
    }

    pub fn mem_loader(&mut self, file_path: &str) {
        self.ram.loader(file_path);
    }

    /* Debug Usage Functions:
     *
     */

    pub fn show_mem(&self) {
        self.ram.show_mem();
    }
}
