#[derive(Debug)]
pub struct RegFile {
    registers: [u32; 32],
}

impl RegFile {
    pub fn new() -> Self {
        Self { registers: [0; 32] }
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
    pub fn show_regs(&self) {}
}

#[derive(Debug, Default)]
pub struct IF2ID {
    instruction_address: usize,
    instruction: u32,
}

#[derive(Debug, Default)]
pub struct ID2EX {
    instruction_address: usize,
    instruction: u32,
    aluop1src: bool,
    aluop2src: bool,
    immediate: u32,
    mem_read_en: bool,
    mem_write_en: bool,
    reg_write_src: u32,
    reg_write_en: bool,
    reg_write_address: usize,
}

#[derive(Debug, Default)]
pub struct EX2MEM {}

#[derive(Debug, Default)]
pub struct MEM2WB {}
