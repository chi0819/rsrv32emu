pub enum MSTATE {
    UNPRIVILEGE,
    PRIVILEGE,
    MACHINE
}

impl MSTATE {
    pub fn read(&self) {
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
}

pub struct CPU {
    pc: u32,
    state: MSTATE,
    registers: RegFile
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            state: MSTATE::UNPRIVILEGE,
            registers: RegFile::new()
        }
    }

    pub fn run(&mut self, ram: &mut [u8]) {
        println!("Hello");
    }
}
