use std::{os, float};

fn main() {
    let args: ~[~str] = os::args();
    let mut i = 1;
    let mut n = 0;
    let mut total = 0.0;
    while i < args.len() {
	total += match float::from_str(args[i]) {
	    Some(k)	=> { n += 1; k }
	    None	=> { println(fmt!("Bad Input: %s", args[i]) ); 0.0 }
	};
	i += 1;
    }
    println("Average: " + (total/(n as float)).to_str());
}
