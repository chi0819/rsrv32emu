use std::env;
use std::io;
use std::fs;

mod cpu;
mod preprocess;
use cpu::*;
use preprocess::*;

const RAM_SIZE: usize = 64 * 1024;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let option: Option<Mode> = match args[2].as_str() {
        "run" => Some(Mode::RUN),
        "show" => Some(Mode::SHOWINSTRUCTION),
        _ => None
    };

    if &args[1] != "all" {
        let mut ram = vec![0u8; RAM_SIZE];
        let mut cpu = CPU::new();
        process_file(&format!("bin/{}.bin", &args[1]), &mut cpu, &mut ram, &option)?;
    } else {
        let dir_path = "bin";
        println!("No specific file provided. Processing all files in directory: {}", dir_path);

        let paths = fs::read_dir(dir_path)?;
        for path in paths {
            if let Ok(entry) = path {
                let file_path = entry.path();
                let mut ram = vec![0u8; RAM_SIZE];
                let mut cpu = CPU::new();
                process_file(file_path.to_str().unwrap(), &mut cpu, &mut ram, &option)?;
            }
        }
    }

    Ok(())
}
