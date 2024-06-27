use clap::{arg, Parser};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// the pattern to search for
    query: String,
    /// the path to the file to read
    file_path: std::path::PathBuf,
}


fn main() {
    // let query = std::env::args().nth(1).expect("no query or search pattern provided");
    // let file_path = std::env::args().nth(2).expect("no file path provided");
    //
    // println!("pattern: {:?}, file_path: {:?}", query, file_path);
    //
    // let args = Cli {
    //     query,
    //     file_path: std::path::PathBuf::from(file_path),
    // };

    let args = Cli::parse();

    println!("pattern: {:?}, file_path: {:?}", args.query, args.file_path);

    let content = std::fs::read_to_string(args.file_path).expect("could not read file");


    print!("Hits: \n");
    for line in content.lines() {
        if line.contains(&args.query) {
            println!("{line}");
        }
    }
}
