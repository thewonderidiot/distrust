#[link(name = "distrust",
       vers = "0",
       url  = "http://github.com/thewonderidiot/distrust")];
#[desc = "A simple disassembler in Rust"];
#[license = "MIT"];
#[crate_type = "bin"];

extern mod extra;
use extra::getopts::{getopts, optflag};
use std::path::Path;
use std::os;
use std::io;

fn print_usage() {
    println("Usage: distrust [OPTIONS] INPUT\n");
    println("Options:");
    println("    -h --help    Display this message");
    println("");
}

fn main() {
    let args = os::args();
    if args.len() == 1 {
        print_usage();
        return;
    }

    let opts = [
        optflag("h"),
        optflag("help")
    ];
    let matches = getopts(args.tail(), opts).unwrap();

    if matches.opt_present("h") || matches.opt_present("help") {
        print_usage();
        return;
    }
    // process more (actually useful) arguments...

    let file = match matches.free.len() {
        0u => fail!("no filename supplied"),
        1u => io::file_reader(&Path(matches.free[0])).unwrap(),
        _  => fail!("multiple input filenames")
    };

    println!("Successfully opened {}", matches.free[0]);
}
