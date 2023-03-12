use ego_tree::iter::Edge;
use regex::Regex;
use scraper::Node;
use scraper::{ElementRef, Html};

use crate::types::doc_node::DocNode;

/**
 * We apply special treatment to <hr/> tags found in the raw HTML.
 * The parser forces all open tags closed as soon as we discover an hr.
 * We want to leave these open so that we can try and retain our nested node structured
 * as best as possible.
 *
 * We temporarily replace these with the hrbr tag to avoid this parser behavior and swap these back after
 * parsing.
 */
static HRBR_PLACEHOLDER: &str = "hrbr";
static PLAIN_BLOCK_LEVEL_ELEMENTS: [&str; 2] = ["div", "p"];

pub fn esc_hr(hrstr: String) -> String {
    let re = Regex::new(r"</?hr/?>").unwrap();
    return re
        .replace_all(&hrstr, format!("<{HRBR_PLACEHOLDER}></{HRBR_PLACEHOLDER}>"))
        .to_string();
}

/**
 * Squish surrounding whitespace to a single space if it exists.
 */
pub fn squish_surrounding_whitespace(input: &str) -> String {
    let re = Regex::new(r"^\s+|\s+$").unwrap();
    re.replace_all(input, " ").to_string()
}

/**
 * We parse a raw scraper::HTML and return a
 * list of leaf doc nodes  (each with a linked list pointer to the root)
 * for us to attempt to transform into an ADF Document
 */
pub fn extract_leaves(fragment: &Html) -> Vec<DocNode> {
    let mut leaf_nodes: Vec<DocNode> = Vec::new();
    fragment
        .root_element()
        .traverse()
        .for_each(|edge| match edge {
            Edge::Close(node) => {
                if let Some(element) = ElementRef::wrap(node) {
                    if element.value().name() == "iframe" || element.value().name() == "img" {
                        leaf_nodes.push(DocNode {
                            name: element.value().name().trim(),
                            text: "".trim().to_owned(),
                            node,
                        })
                    } else if element.value().name() == HRBR_PLACEHOLDER {
                        leaf_nodes.push(DocNode {
                            name: "hr",
                            text: "".trim().to_owned(),
                            node,
                        })
                    }
                } else if let Node::Text(text_node) = node.value() {
                    if let Some(parent) = node.parent().and_then(ElementRef::wrap) {
                        if PLAIN_BLOCK_LEVEL_ELEMENTS.contains(&parent.value().name()) || !text_node.text.trim().is_empty() {
                            leaf_nodes.push(DocNode {
                                name: "text",
                                text: squish_surrounding_whitespace(&text_node.text),
                                node,
                            })
                        }
                    }
                }
            }
            Edge::Open(_) => (),
        });
    leaf_nodes
}
