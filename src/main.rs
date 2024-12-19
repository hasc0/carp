use std::env;

mod strip;
use strip::{handler, Opts};

mod utils;
use utils::{confirm_overwrite, parse_opts};

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: print list of options/flags following line describing the problem
    // TODO: add way to set output directory
    if args.len() < 3 {
        println!("Input file and option flag required.");
        return;
    } else if args.len() > 4 {
        println!("Too many arguments provided.");
        return;
    }

    if args[1].get(0..1) != Some("-") || args[0].len() < 2 {
        println!("Please supply a valid option flag.");
        return;
    }

    let flag: &str = &args[1];
    let opts: Vec<Opts> = parse_opts(flag);
    if opts.len() == 0 {
        println!("Invalid option flag provided.");
        return;
    }

    let infile: String = args[2].to_owned();
    let infile_split: Vec<&str> = infile.split(".").collect();
    if infile_split.len() < 2 {
        println!("Invalid filename.");
        return;
    }

    // TODO: currently unused (_), check that the filetype of the outfile name matches the infile, or allow conversions!
    let _filetype: String = infile_split[infile_split.len() - 1].to_owned();

    let mut outfile: Option<String> = None;
    if args.len() == 3 {
        // TODO: if return val is false, prompt user to submit an outfile name
        match confirm_overwrite() {
            true => println!("{} will be overwritten.", infile),
            false => return,
        }
    } else {
        outfile = Some(args[3].clone());
    }

    handler(infile, outfile, opts);
}
