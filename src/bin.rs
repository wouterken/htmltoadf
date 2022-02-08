use std::io::Read;
use std::io::stdin;
use htmltoadf::convert_html_str_to_adf_str;

/**
 * Main function. Convert an HTML string from STDIN to an ADF string
 * and print to STDOUT.
 */
fn main() {
    let adf = convert_html_str_to_adf_str(read_html());
    println!("{adf}");
}

/**
 * Read HTML input from STDIN
 */
pub fn read_html()-> String {
  let mut html = String::new();
  stdin().lock().read_to_string(&mut html).expect("Read from STDIN failed");
  return html
}
