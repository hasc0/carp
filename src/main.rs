use std::env;

mod strip;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // TODO: print list of options/flags following line describing the problem
        println!("Input file required.");
        return;
    }

    let filename: String = args[1].to_owned();
    let filename_split: Vec<&str> = filename.split(".").collect();
    if filename_split.len() < 2 {
        println!("Invalid filename.");
        return;
    }

    let filetype: String = filename_split[filename_split.len() - 1].to_owned();

    // TODO: replace println!() with function calls for respective filetype (need to be implemented)
    // determine if or (||) is possible in match statements
    match filetype.as_str() {
        "txt" => crate::strip::txt_gen(filename, filetype),
        "csv" => crate::strip::csv_gen(filename, filetype),
        _ => println!("File type may not be recognized or supported (currently)."),
    }
}
