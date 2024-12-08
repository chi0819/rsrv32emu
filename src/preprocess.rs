use std::io::{self, Read};
use std::path::Path;
use std::fs::File;
use crate::cpu::*;


pub enum Mode {
    SHOWINSTRUCTION,
    RUN,
}

pub fn process_file(file_path: &str, cpu: &mut CPU, ram: &mut [u8], option: &Option<Mode>) -> io::Result<()> {
    let path = Path::new(file_path);
    println!("Processing file: {}", file_path);

    let mut file = File::open(path)?;
    file.read(ram)?;

    match option {
        Some(Mode::SHOWINSTRUCTION) => {
            println!("RISC-V Instructions:");
            for chunk in ram.chunks(4) {
                if chunk.len() == 4 {
                    let instruction = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    println!("0x{:08x}", instruction);
                } else {
                    eprintln!("Warning: Incomplete instruction at the end of the RAM");
                }
            }
        },
        Some(Mode::RUN) => {
            cpu.run(ram);
        },
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "No mode provided"))
    }

    Ok(())
}
