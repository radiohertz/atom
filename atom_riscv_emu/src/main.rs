
use atom_riscv::{core::{hart::{Hart}}};
use std::env::{args};
use std::time::Instant;
use std::fs::{read};

fn main() {
    let p_args = args().collect::<Vec<String>>();

    if p_args.len() < 2 {
        panic!("Usage: atom [filepath]")
    }

    let filename = &p_args[1];
    let code = read(filename).unwrap();

    let timer = Instant::now();

    let mut hart = Hart::new(Some(code));
    hart.run();

    let time_taken = timer.elapsed();
    hart.debug();
    println!("Time taken to run the prog: {:?}", time_taken);
}
