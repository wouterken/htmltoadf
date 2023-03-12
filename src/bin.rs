use clap::Parser;
use htmltoadf::convert_html_str_to_adf_str;
use std::fs;
use std::io::stdin;
use std::io::Read;

/// Convert the given file to adf
#[derive(Parser)]
#[clap(author="Wouter Coppieters", version="0.1.7", about = None, long_about = None)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    inpath: Option<std::path::PathBuf>,
    // /// The path to the file to output
    #[clap(short, long)]
    outpath: Option<std::path::PathBuf>,
}

///
/// Main function. Convert an HTML string from input file or STDIN to an ADF string
/// and write to output file if given, else print to STDOUT.
///
fn main() {
    let cli = Cli::parse();

    //Read HTML from input file if given, otherwise STDIN
    let adf = if let Some(config_path) = cli.inpath.as_deref() {
        let contents =
            fs::read_to_string(config_path).expect("Something went wrong reading the input file");
        convert_html_str_to_adf_str(contents)
    } else {
        convert_html_str_to_adf_str(read_html())
    };

    if let Some(outpath) = cli.outpath.as_deref() {
        fs::write(outpath, adf).expect("Something went wrong writing output file");
    } else {
        println!("{adf}");
    };
}

/**
 * Read HTML input from STDIN
 */
pub fn read_html() -> String {
    let mut html = String::new();
    stdin()
        .lock()
        .read_to_string(&mut html)
        .expect("Read from STDIN failed");
    html
}
