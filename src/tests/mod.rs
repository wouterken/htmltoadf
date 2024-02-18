mod colors;
#[cfg(test)]
mod combination;
mod empty;
mod headings;
mod image;
mod lists;
mod marks;
mod paragraphs;
mod tables;
use crate::convert_html_str_to_adf_str;

#[allow(dead_code)]
fn assert_output_json_eq(html: &str, expected: serde_json::Value) {
    let converted = convert_html_str_to_adf_str(html.to_string());
    assert_eq!(expected.to_string(), converted);
}
