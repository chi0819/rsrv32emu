use std::env;
use std::io;
use std::fs;

/* Declare Modules */
pub mod riscv;
pub mod cpu;
pub mod peripheral;
pub mod preprocess;

/* Import Modules */
use cpu::*;
use preprocess::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let option: Option<Mode> = match args[2].as_str() {
        "run" => Some(Mode::RUN),
        "show" => Some(Mode::SHOWINSTRUCTION),
        _ => None
    };

    if args[1].as_str() != "all" {
        let mut cpu = CPU::new();
        process_file(&format!("bin/{}.bin", &args[1]), &mut cpu, &option)?;
    } else {
        let dir_path = "bin";
        let paths = fs::read_dir(dir_path)?;
        for path in paths {
            if let Ok(entry) = path {
                let file_path = entry.path();
                let mut cpu = CPU::new();
                process_file(file_path.to_str().unwrap(), &mut cpu, &option)?;
            }
        }
    }

    Ok(())
}
