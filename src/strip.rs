// strip module defined in ./main.rs

use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::Write;

// first three are self-explanatory, interstitial is for removing empty newlines (lines of length 0)
pub enum Opts {
    All,
    Leading,
    Trailing,
    Interstitial,
}

pub fn handler(infile: String, outfile: Option<String>, opts: Vec<Opts>) {
    let newfile: String = outfile.unwrap_or(infile.clone());
    let mut lines: Vec<String> = parse(&infile);

    for opt in opts {
        match opt {
            Opts::All => astrip(&mut lines),
            Opts::Leading => lstrip(&mut lines),
            Opts::Trailing => tstrip(&mut lines),
            Opts::Interstitial => istrip(&mut lines),
        }
    }

    let _ = write(newfile, lines);
}

fn parse(file: &String) -> Vec<String> {
    let data: String = fs::read_to_string(file).expect("File should have been read.");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut clean_lines: Vec<String> = vec![];
    for line in lines {
        let temp_line: String = line.trim_end_matches('\r').to_owned();
        clean_lines.push(temp_line);
    }

    return clean_lines;
}

fn astrip(lines: &mut Vec<String>) {
    for line in &mut *lines {
        let temp_line: String = line.trim().to_owned();
        *line = temp_line;
    }

    istrip(lines);
}

fn lstrip(lines: &mut Vec<String>) {
    for line in lines {
        let temp_line: String = line.trim_start().to_owned();
        *line = temp_line;
    }
}

fn tstrip(lines: &mut Vec<String>) {
    for line in lines {
        let temp_line: String = line.trim_end().to_owned();
        *line = temp_line;
    }
}

fn istrip(lines: &mut Vec<String>) {
    let mut new_lines: Vec<String> = vec![];
    for line in &mut *lines {
        if !line.is_empty() {
            new_lines.push(line.to_owned());
        }
    }

    *lines = new_lines;
}

fn write(outfile: String, lines: Vec<String>) -> Result<()> {
    let mut file = File::create(outfile.clone())?;
    for line in lines {
        writeln!(&mut file, "{line}")?;
    }

    let _ = file.flush();

    println!("Success! Output Filename: {}", outfile);
    Ok(())
}
