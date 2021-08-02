
use atom_riscv::{VERSION,core::{hart::{Hart}}};
use std::env::{args};
use std::fs::{read};

fn main() {
    let p_args = args().collect::<Vec<String>>();

    if p_args.len() < 2 {
        panic!("Usage: atom [filepath]")
    }
    let filename = &p_args[1];

    let code = read(filename).unwrap();
    let mut hart = Hart::new(Some(code));
    hart.run();
    hart.debug();
}