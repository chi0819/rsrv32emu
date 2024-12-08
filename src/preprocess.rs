use std::io;
use crate::cpu::*;

pub enum Mode {
    SHOWINSTRUCTION,
    RUN,
}

pub fn process_file(file_path: &str, cpu: &mut CPU, option: &Option<Mode>) -> io::Result<()> {
    cpu.mem_loader(file_path);

    match option {
        Some(Mode::SHOWINSTRUCTION) => cpu.show_mem(),
        Some(Mode::RUN) => cpu.run(),
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "No mode provided"))
    }

    Ok(())
}
