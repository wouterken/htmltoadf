use serde_json::Value;
use std::collections::HashMap;

use crate::types::{
    adf_content_type::{AdfContentType, AdfMark, AdfMarkAttributes},
    adf_permitted_children::AdfPermittedChildren,
};

lazy_static! {
  /**
   * Placeholder empty child type. Does not permit any child types.
   */
  pub static ref EMPTY_CHILD_TYPE: AdfPermittedChildren = AdfPermittedChildren::any(&[]);

  /**
   * 1. LEGAL_CHILD_TYPES: The allowed nesting of ADF Types that we permit in our output doc.
   */
  pub static ref LEGAL_CHILD_TYPES: HashMap<String, AdfPermittedChildren> = HashMap::from([
    (
      String::from("paragraph"),
      AdfPermittedChildren::any(&["text", "emoji", "hardBreak"])
    ),
    (
      String::from("heading"),
      AdfPermittedChildren::any(&["text", "emoji", "hardBreak"])
    ),
    (
      String::from("bulletList"),
      AdfPermittedChildren::any(&["listItem"])
    ),
    (
      String::from("orderedList"),
      AdfPermittedChildren::any(&["listItem"])
    ),
    (
      String::from("blockquote"),
      AdfPermittedChildren::any(&["paragraph"])
    ),
    (
      String::from("codeBlock"),
      AdfPermittedChildren::any(&["paragraph"])
    ),
    (
      String::from("listItem"),
      AdfPermittedChildren::any_starts_with(&["paragraph", "mediaSingle", "codeBlock"], &["paragraph", "mediaAdfPermittedChildren", "codeBlock", "orderedList", "bulletList"])
    ),
    (
      String::from("table"),
      AdfPermittedChildren::any(&["tableRow"])
    ),
    (
      String::from("tableRow"),
      AdfPermittedChildren::any(&["tableHeader","tableCell"])
    ),
    (
      String::from("tableHeader"),
      AdfPermittedChildren::any(&["codeBlock", "blockCard", "paragraph", "bulletList", "mediaSingle", "orderedList", "heading", "panel", "blockquote", "rule", "mediaGroup", "decisionList", "taskList", "extension", "embedCard", "nestedExpand"])
    ),
    (
      String::from("tableCell"),
      AdfPermittedChildren::any(&["codeBlock", "blockCard", "paragraph", "bulletList", "mediaSingle", "orderedList", "heading", "panel", "blockquote", "rule", "mediaGroup", "decisionList", "taskList", "extension", "embedCard", "nestedExpand", "hardBreak"])
    ),
    (
      String::from("doc"),
      AdfPermittedChildren::any(&["blockCard", "blockquote", "bodiedExtension", "bulletList", "codeBlock", "decisionList", "embedCard", "expand", "extension", "heading", "layoutSection", "mediaGroup", "mediaSingle", "orderedList", "panel", "paragraph", "rule", "table", "taskList"])
    )
  ]);

  #[derive(Debug, Clone)]
  /**
   * NODE_MAP: The legal mappings from HTML element types to ADF types that we permit.
   */
  pub static ref NODE_MAP: HashMap<&'static str, AdfContentType> = HashMap::from([
    (
      "p",
      AdfContentType::from_name("paragraph")
    ),
    (
      "blockquote",
      AdfContentType::from_name("blockquote")
    ),
    (
      "span",
      AdfContentType::from_name("text")
    ),
    (
      "text",
      AdfContentType::from_name("text")
    ),
    (
      "ul",
      AdfContentType::from_name("bulletList")
    ),
    (
      "ol",
      AdfContentType::from_name("orderedList")
    ),
    (
      "li",
      AdfContentType::from_name("listItem")
    ),
    (
      "hr",
      AdfContentType::from_name("rule")
    ),
    (
      "br",
      AdfContentType::from_name("hardBreak")
    ),
    (
      "html",
      AdfContentType::from_name("doc")
    ),
    (
      "body",
      AdfContentType::from_name("doc")
    ),
    (
      "table",
      AdfContentType::from_name("table")
    ),
    (
      "tr",
      AdfContentType::from_name("tableRow")
    ),
    (
      "th",
      AdfContentType::from_name("tableHeader")
    ),
    (
      "td",
      AdfContentType::from_name("tableCell")
    ),
    (
      "iframe",
      AdfContentType::from_name_and_attributes("paragraph", |node|{
        match node.value().attr("src"){
          Some(attribute) => vec![("src".to_string(), Value::String(attribute.to_string()))],
          None => vec!()
        }
      }
      )
    ),
    (
      "b",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "strong".to_string(),
          attributes: AdfMarkAttributes::List(vec!())
        }
      ])
    ),
    (
      "strong",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "strong".to_string(),
          attributes: AdfMarkAttributes::List(vec!())
        }
      ])
    ),
    (
      "i",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "em".to_string(),
          attributes: AdfMarkAttributes::List(vec!())
        }
      ])
    ),
    (
      "em",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "em".to_string(),
          attributes: AdfMarkAttributes::List(vec!())
        }
      ])
    ),
    (
      "u",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "underline".to_string(),
          attributes: AdfMarkAttributes::List(vec!())
        }
      ])
    ),
    (
      "code",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "code".to_string(),
          attributes: AdfMarkAttributes::List(vec!())
        }
      ])
    ),
    (
      "a",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "link".to_string(),
          attributes: AdfMarkAttributes::Generator(|element| -> Vec<(String, String)>{
            match element.value().attr("href"){
              Some(attribute) => vec![
                ("href".to_string(), attribute.to_string())
              ],
              None => vec!()
            }
          })
        }
      ])
    ),
    (
      "sub",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "subsup".to_string(),
          attributes: AdfMarkAttributes::List(vec![("type".to_string(), "sub".to_string())])
        }
      ])
    ),
    (
      "sup",
      AdfContentType::from_name_and_marks("text", &[
        AdfMark{
          typename: "subsup".to_string(),
          attributes: AdfMarkAttributes::List(vec![("type".to_string(), "sup".to_string())])
        }
      ])
    ),
    (
      "h1",
      AdfContentType::from_name_and_attributes("heading", |_|
        {
          vec!(
            ("level".to_string(), Value::Number(serde_json::Number::from(1))),
          )
        }
      )
    ),
    (
      "h2",
      AdfContentType::from_name_and_attributes("heading", |_|
        {
          vec!(
            ("level".to_string(), Value::Number(serde_json::Number::from(2))),
          )
        }
      )
    ),
    (
      "h3",
      AdfContentType::from_name_and_attributes("heading", |_|
        {
          vec!(
            ("level".to_string(), Value::Number(serde_json::Number::from(3))),
          )
        }
      )
    ),
    (
      "h4",
      AdfContentType::from_name_and_attributes("heading", |_|
        {
          vec!(
            ("level".to_string(), Value::Number(serde_json::Number::from(4))),
          )
        }
      )
    ),
    (
      "h5",
      AdfContentType::from_name_and_attributes("heading", |_|
        {
          vec!(
            ("level".to_string(), Value::Number(serde_json::Number::from(5))),
          )
        }
      )
    ),
    (
      "h6",
      AdfContentType::from_name_and_attributes("heading", |_|
        {
          vec!(
            ("level".to_string(), Value::Number(serde_json::Number::from(6))),
          )
        }
      )
    ),
    (
      "img",
      AdfContentType::from_name_and_attributes("mediaSingle", |node|
        {
          match node.value().attr("src"){
            Some(attribute) => vec!(
              ("url".to_string(), Value::String(attribute.to_string())),
              ("type".to_string(), Value::String("external".to_string()))
            ),
            None => vec!()
          }

        }
      )
    ),
]);
}
