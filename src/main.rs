use clap::{Arg, ArgAction, Command};
use std::env;
use std::fs;
use std::io;

/* Declare Modules */
pub mod riscv;
pub mod cpu;
pub mod cores;
pub mod peripheral;
pub mod utils;
pub mod entry;

/* Import Modules */
use cpu::*;
use entry::*;
use riscv::*;

fn main() -> io::Result<()> {
    let matches = Command::new("Rust RISC-V 32 Emulator")
        .about("RSRV32EMU Options")
        .arg(
            Arg::new("run")
                .long("run")
                .help("run specific binary code")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show")
                .long("show")
                .help("show the disassembled binary code with specific binary file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .help("user choose target binary file")
                .value_name("SELECT BINARY FILE IN THE BIN/")
                .num_args(1)
                .required(false),
        )
        .get_matches();

    if matches.get_flag("show") {
        match matches.get_one::<String>("file") {
            Some(filename) => {
                let mut cpu = CPU::new();
                entry(
                    &format!("bin/{}.bin", &filename),
                    &mut cpu,
                    &Mode::SHOWINSTRUCTION,
                )?;
            }
            Some(_) | None => {
                eprintln!("Error: In show mode must specify the target binary file");
                std::process::exit(1);
            }
        }
    } else if matches.get_flag("run") {
        match matches.get_one::<String>("file") {
            Some(filename) => {
                let mut cpu = CPU::new();
                entry(
                    &format!("bin/{}.bin", &filename),
                    &mut cpu,
                    &Mode::RUN,
                )?;
            }
            Some(_) | None => {
                eprintln!("Error: In run mode must specify the target binary file");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Please select which mode you want to run also provide the target binary file");
        std::process::exit(1);
    }

    Ok(())
}
