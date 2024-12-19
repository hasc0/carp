// utils module defined in ./main.rs

use std::io;
use std::io::Read;

use super::strip::Opts;

pub fn confirm_overwrite() -> bool {
    println!("This operation will overwrite the original file. Proceed? ([y]es/[n]o)");

    loop {
        let mut ans = [0];
        let _ = io::stdin().read(&mut ans);
        match ans[0] as char {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => println!("Enter your response as y/Y/n/N"),
        }
    }
}

pub fn parse_opts(flag: &str) -> Vec<Opts> {
    let mut opts: Vec<Opts> = vec![];

    match flag {
        "--all" => opts.push(Opts::All),
        "--leading" => opts.push(Opts::Leading),
        "--trailing" => opts.push(Opts::Trailing),
        "--interstitial" => opts.push(Opts::Interstitial),
        "-a" => opts.push(Opts::All),
        "-l" => opts.push(Opts::Leading),
        "-t" => opts.push(Opts::Trailing),
        "-i" => opts.push(Opts::Interstitial),
        "-lt" => {
            opts.push(Opts::Leading);
            opts.push(Opts::Trailing);
        }
        "-li" => {
            opts.push(Opts::Leading);
            opts.push(Opts::Interstitial);
        }
        "-ti" => {
            opts.push(Opts::Trailing);
            opts.push(Opts::Interstitial);
        }
        _ => return opts,
    }

    return opts;
}
