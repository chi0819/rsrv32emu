use crate::cpu::*;
use crate::riscv::*;
use std::io;

pub fn entry(file_path: &str, cpu: &mut CPU, option: &Mode) -> io::Result<()> {
    cpu.mem_loader(file_path);

    match option {
        Mode::SHOWINSTRUCTION => cpu.show_mem(),
        Mode::RUN => cpu.run(),
    }

    Ok(())
}
