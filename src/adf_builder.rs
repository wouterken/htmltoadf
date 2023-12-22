use crate::extractor;
use crate::types::adf_content_type::content_type_for_node_type;
use crate::types::adf_content_type::is_valid_child_type;
use crate::types::adf_content_type::AdfMark;
use crate::types::adf_content_type::AdfMarkAttributes;
use crate::types::doc_node::DocNode;
use crate::types::node_list::NodeHandle;
use crate::types::node_list::NodeList;
use regex::Regex;
use scraper::ElementRef;
use scraper::Html;
use serde_json::{Map, Value};

static VALID_EMPTY_TYPES: [&str; 4] = ["hr", "iframe", "img", "br"];

/**
* The main procedure for our ADF Builder.
* Allows us to take an HTML string and turn it into a valid ADF string.
*
*/

/// ```rust
/// use htmltoadf::convert_html_str_to_adf_str;
/// use serde_json::json;
///
/// let converted = convert_html_str_to_adf_str("<h1>Hello World</h1>".to_string());
/// let expected = json!({
///     "version": 1,
///     "type": "doc",
///     "content": [
///         {
///             "type": "heading",
///             "attrs": {
///                 "level": 1
///             },
///             "content": [
///                 {
///                     "type": "text",
///                     "text": "Hello World"
///                 }
///             ]
///         }
///     ]
/// }).to_string();
///assert_eq!(expected, converted);
/// ```

pub fn convert_html_str_to_adf_str(html: String) -> String {
    let fragment = Html::parse_fragment(&extractor::esc_hr(html));
    let leaf_nodes = extractor::extract_leaves(&fragment);
    let node_list = build_adf_doc(leaf_nodes);
    node_list.to_json()
}

/**
 * Accept a list of leaf nodes, and turns it into a fully populated NodeList - representing the structure of our ADF Doc.
 */
fn build_adf_doc(leaf_nodes: Vec<DocNode>) -> NodeList {
    let mut node_list = Default::default();
    let mut current_paragraph_handle: NodeHandle = 0;

    leaf_nodes.iter().for_each(|leaf| {
        if leaf.text.is_empty() && !VALID_EMPTY_TYPES.contains(&leaf.name) {
            return;
        }
        let content_type = content_type_for_node_type(leaf.name);
        let (parent, marks) = build_parent_path(leaf, &mut node_list);
        let mut insertion_point = find_valid_insertion_point(leaf, parent, &mut node_list);
        let insertion_node = node_list.node(insertion_point);

        let attributes = match content_type.attributes {
            Some(attribute_generator) => attribute_generator(&ElementRef::wrap(leaf.node).unwrap()),
            _ => vec![],
        };

        match leaf.name {
            "img" => {
                let media_group_handle = node_list.push_anon(
                    insertion_point,
                    content_type.typename.to_string(),
                    "".to_string(),
                    &[],
                    vec![],
                );
                node_list.push_anon(
                    media_group_handle,
                    "media".to_string(),
                    "".to_string(),
                    &attributes,
                    vec![],
                );
            }
            "iframe" => {
                let paragraph_handle = node_list.push_anon(
                    insertion_point,
                    "paragraph".to_string(),
                    "".to_string(),
                    &[],
                    vec![],
                );
                let src = attributes
                    .clone()
                    .into_iter()
                    .find(|attr| attr.0.eq("src"))
                    .unwrap_or(("".to_string(), Value::String("".to_string())))
                    .1;
                let mut marks: Vec<Value> = vec![];
                insert_adf_mark(
                    &mut marks,
                    String::from("link"),
                    vec![("href".to_string(), src.to_string())],
                );
                node_list.push_anon(
                    paragraph_handle,
                    "text".to_string(),
                    "".to_string(),
                    &[(
                        "text".to_string(),
                        Value::String("External Content".to_string()),
                    )],
                    marks,
                );
            }
            "hr" => {
                node_list.push_anon(
                    insertion_point,
                    content_type.typename.to_string(),
                    "".to_string(),
                    &[],
                    vec![],
                );
            }
            "p" => {
                let paragraph_handle = node_list.push_anon(
                    insertion_point,
                    content_type.typename.to_string(),
                    "".to_string(),
                    &[],
                    vec![],
                );
                node_list.push_anon(
                    paragraph_handle,
                    "text".to_string(),
                    leaf.text.to_string(),
                    &attributes,
                    marks,
                );
            }
            _ => {
                // Text nodes must sometimes be wrapped in a paragraph to be valid.
                // If we are a text node and our immediate parent only supports a paragraph we wrap this node inside a paragraph
                // If a sibling node has already wrapped itself in a paragraph, we will try to use the same paragraph
                if (insertion_node.is_none()
                    || !is_valid_child_type(
                        &insertion_node.unwrap().node_type,
                        "text",
                        0,
                    )
                    || insertion_point == 1)
                    && (
                        content_type.typename.eq("text") ||
                        content_type.typename.eq("hardBreak")
                    )
                {
                    let parent_node = node_list.node(parent);
                    if parent_node.is_some()
                        && is_valid_child_type(&parent_node.unwrap().node_type, "paragraph", 0)
                    {
                        insertion_point = parent
                    }
                    let insertion_node = node_list.node(insertion_point);
                    current_paragraph_handle = if current_paragraph_handle != 0
                        && insertion_node.is_some()
                        && *insertion_node.unwrap().children.last().unwrap_or(&0)
                            == current_paragraph_handle
                    {
                        current_paragraph_handle
                    } else {
                        node_list.push_anon(
                            insertion_point,
                            "paragraph".to_string(),
                            "".to_string(),
                            &[],
                            vec![],
                        )
                    };
                    if content_type.typename.eq("text") {
                        node_list.push_anon(
                            current_paragraph_handle,
                            content_type.typename.to_string(),
                            leaf.text.to_string(),
                            &attributes,
                            marks,
                        );
                    }
                    else{
                        node_list.push_anon(
                            current_paragraph_handle,
                            content_type.typename.to_string(),
                            leaf.text.to_string(),
                            &attributes,
                            vec![],
                        );
                    }
                } else if insertion_node.is_some()
                    && is_valid_child_type(
                        &insertion_node.unwrap().node_type,
                        &content_type.typename,
                        insertion_node.unwrap().children.len(),
                    )
                {
                    if !leaf.text.to_string().is_empty() && !content_type.typename.eq("text") {
                        let parent_handle = node_list.push_anon(
                            insertion_point,
                            content_type.typename.to_string(),
                            "".to_string(),
                            &attributes,
                            vec![],
                        );
                        if is_valid_child_type(
                            &content_type.typename.to_string(),
                            "text",
                            0,
                        ) {
                            node_list.push_anon(
                                parent_handle,
                                "text".to_string(),
                                leaf.text.to_string(),
                                &[],
                                marks,
                            );
                        } else {
                            let wrapper_para_handle = node_list.push_anon(
                                parent_handle,
                                "paragraph".to_string(),
                                "".to_string(),
                                &[],
                                vec![],
                            );
                            node_list.push_anon(
                                wrapper_para_handle,
                                "text".to_string(),
                                leaf.text.to_string(),
                                &[],
                                marks,
                            );
                        }
                    } else {
                        node_list.push_anon(
                            insertion_point,
                            content_type.typename.to_string(),
                            leaf.text.to_string(),
                            &attributes,
                            marks,
                        );
                    }
                }
            }
        }
    });
    node_list
}

/**
 *  Inserts the new mark using valid ADF structure into our list.
 */
fn insert_adf_mark(marks: &mut Vec<Value>, typename: String, pairs: Vec<(String, String)>) {
    let mut mark_json = Map::new();
    mark_json.insert("type".to_string(), serde_json::Value::String(typename));

    if !pairs.is_empty() {
        mark_json.insert("attrs".to_string(), serde_json::Value::Object(Map::new()));

        pairs.iter().for_each(|pair| {
            mark_json[&"attrs".to_string()][pair.0.clone()] =
                serde_json::Value::String(pair.1.clone());
        });
    }

    marks.push(serde_json::Value::Object(mark_json))
}

/**
 * Builds a mark (Value) from an AdfMark, and adds it to a list of marks.
 */
fn insert_mark_value(marks: &mut Vec<Value>, mark: &AdfMark, node: &ElementRef) {
    let pair_list: Vec<(String, String)> = match &mark.attributes {
        AdfMarkAttributes::List(pairs) => pairs.clone(),
        AdfMarkAttributes::Generator(lambda) => {
            lambda(node)
        }
    };
    insert_adf_mark(marks, mark.typename.clone(), pair_list);
}

/**
 * Generates a list of styles from a single node.
 */
fn extract_styles(node: &ElementRef) -> Option<Vec<Vec<String>>> {
    node.value().attr("style").map(|inline_style| {
        let styles: Vec<_> = inline_style
            .split(';')
            .map(|p| {
                return p.split(':').map(|pair| pair.trim().to_string()).collect();
            })
            .filter(|p: &Vec<String>| p.len() == 2)
            .collect();
        styles
    })
}

/**
 * Convert a hex or RGB(a) CSS string into a 6 character hex string to be used as a color
 * code in our ADF doc.
 */
fn hex_code_for_color_str(color_str: String) -> Option<String> {
    lazy_static! {
        static ref FULLHEX: Regex = Regex::new(r"(?i)#([0-9A-F]{6})").unwrap();
        static ref HALFHEX: Regex = Regex::new(r"(?i)#([0-9A-F])([0-9A-F])([0-9A-F])").unwrap();
        static ref RGB: Regex =
            Regex::new(r"(?i)RGB\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
        static ref RGBA: Regex =
            Regex::new(r"(?i)RGBA\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    }
    if FULLHEX.is_match(&color_str) {
        let captures = FULLHEX.captures(&color_str).unwrap();
        return Some(captures[1].to_string());
    } else if HALFHEX.is_match(&color_str) {
        let captures = HALFHEX.captures(&color_str).unwrap();
        let (r, g, b) = (&captures[1], &captures[2], &captures[3]);
        return Some(format!("{r}{r}{g}{g}{b}{b}"));
    } else if RGB.is_match(&color_str) {
        let captures = RGB.captures(&color_str).unwrap();
        let (r, g, b) = (&captures[1], &captures[2], &captures[3]);
        return Some(
            [r, g, b]
                .map(|x| format!("{:0>2x}", x.parse::<i32>().unwrap()))
                .join(""),
        );
    } else if RGBA.is_match(&color_str) {
        let captures = RGBA.captures(&color_str).unwrap();
        let (r, g, b) = (&captures[1], &captures[2], &captures[3]);
        return Some(
            [r, g, b]
                .map(|x| format!("{:0>2x}", x.parse::<i32>().unwrap()))
                .join(""),
        );
    }
    None
}

/**
 * Build a path to our leaf node inside our node_list.
 * We accumulate marks as we descend into our tree so that our leaf nodes
 * retain any styles or marks that are introduced by its parents.
 *
 * The path must be legal according to our permitted ADF structure.
 * Nodes that do not have a legal insert location are skipped.
 *
 * Returns a NodeHandle (pointer to the immediate parent), and a vector of marks that
 * should apply at this location.
 */
fn build_parent_path(leaf: &DocNode, node_list: &mut NodeList) -> (NodeHandle, Vec<Value>) {
    let mut node = leaf.node;
    let mut parent_path: Vec<ElementRef> = vec![];
    let mut marks: Vec<Value> = vec![];
    let mut current_node_handle = 0;

    loop {
        let parent_node = node.parent().and_then(ElementRef::wrap);
        if parent_node.is_some() {
            parent_path.push(parent_node.unwrap());
            node = *parent_node.unwrap()
        } else {
            break;
        }
    }
    for node in parent_path.iter().rev() {
        let content_type = content_type_for_node_type(node.value().name());
        content_type.marks.iter().for_each(|mark| {
            insert_mark_value(&mut marks, mark, node);
        });
        if let Some(styles) = extract_styles(node) {
            if let Some(color_style) = styles
                .iter()
                .find(|styles| styles.first().unwrap().eq_ignore_ascii_case("color"))
            {
                let color = color_style.get(1).unwrap().to_string();
                if let Some(text_color) = hex_code_for_color_str(color) {
                    insert_adf_mark(
                        &mut marks,
                        String::from("textColor"),
                        vec![(String::from("color"), format!("#{text_color}"))],
                    );
                }
            }
            if let Some(decoration_style) = styles.iter().find(|styles| {
                styles
                    .first()
                    .unwrap()
                    .eq_ignore_ascii_case("text-decoration")
            }) {
                let decoration = decoration_style.get(1).unwrap().to_string();
                if decoration.eq_ignore_ascii_case("underline") {
                    insert_adf_mark(&mut marks, String::from("underline"), vec![]);
                } else if decoration.eq_ignore_ascii_case("line-through") {
                    insert_adf_mark(&mut marks, String::from("strike"), vec![]);
                }
            }
        }

        if let Some(current_node) = node_list.node(current_node_handle) {
            if content_type.typename == "text"
                || !is_valid_child_type(
                    &current_node.node_type,
                    &content_type.typename,
                    current_node.children.len(),
                )
            {
                continue;
            }
        }

        let attributes = if let Some(attribute_generator) = content_type.attributes {
            attribute_generator(node)
        } else {
            vec![]
        };

        match node_list.push(
            node.id(),
            current_node_handle,
            content_type.typename.to_string(),
            "".to_string(),
            attributes,
            vec![],
        ) {
            Ok(next_node_handle) => current_node_handle = next_node_handle,
            Err(next_node_handle) => current_node_handle = next_node_handle,
        }
    }
    remove_illegal_marks(&mut marks);
    (current_node_handle, marks)
}

/**
 * Remove marks from our document not permitted by the ADF JSON Schema.
 */
fn remove_illegal_marks(marks: &mut Vec<Value>) {
    let code_value = serde_json::Value::String("code".to_string());
    let link_value = serde_json::Value::String("link".to_string());
    let legal_code_types = [&code_value, &link_value];
    if marks.iter().any(|m| m["type"].eq(&code_value)) {
        marks.retain(|m| legal_code_types.contains(&&m["type"]));
    }
}

/**
 * Returns the node handle of the best suited node to hold the
 * given leaf node.
 * Starts the search at a given parent handle, and steps up the tree until a valid insertion point is found
 * OR we hit the document root.
 */
pub fn find_valid_insertion_point(
    leaf: &DocNode,
    parent: NodeHandle,
    node_list: &mut NodeList,
) -> NodeHandle {
    let content_type = content_type_for_node_type(leaf.name);
    let mut parent_handle: NodeHandle = parent;
    while let Some(parent_node) = node_list.node(parent_handle) {
        //We are at the top level, must insert here
        if node_list.node(parent_node.parent).is_none() {
            break;
        }

        // We have found a valid insertion point. Good to insert here.
        if is_valid_child_type(&parent_node.node_type, &content_type.typename, 0) {
            break;
        }

        //We are of type text, but can only insert paragraph. This is ok (We will become type paragraph instead)
        if content_type.typename.eq(&String::from("text"))
            && is_valid_child_type(&parent_node.node_type, "paragraph", 0)
        {
            break;
        }

        // Couldn't find a good spot to insert, lets go further up the tree
        let mut prior_handle = parent_handle;
        parent_handle = parent_node.parent;
        node_list.delete(&mut prior_handle);
    }
    parent_handle
}
