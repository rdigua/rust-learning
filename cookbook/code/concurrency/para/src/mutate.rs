//The example uses the rayon crate, which is a data parallelism library for Rust. rayon provides the par_iter_mut method for any parallel iterable data type. This is an iterator-like chain that potentially executes in parallel.
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}

