use std::{os, uint};

fn main() {
    let args: ~[~str] = os::args();
    let mut i = 1;
    while i < args.len() {
	print(args[i] + " ");
	i = i+1;
    }
    println("");
}
