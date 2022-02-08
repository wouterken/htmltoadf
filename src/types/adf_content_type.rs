use crate::adf_structure::EMPTY_CHILD_TYPE;
use crate::adf_structure::LEGAL_CHILD_TYPES;
use crate::adf_structure::NODE_MAP;

use scraper::ElementRef;
use serde_json::Value;

#[derive(Clone)]
pub enum AdfMarkAttributes {
    List(Vec<(String, String)>),
    Generator(fn(&ElementRef) -> Vec<(String, String)>),
}

#[derive(Clone)]
pub struct AdfMark {
    pub typename: String,
    pub attributes: AdfMarkAttributes,
}

#[derive(Clone, Default)]
pub struct AdfContentType {
    pub typename: String,
    pub marks: Vec<AdfMark>,
    pub attributes: Option<fn(&ElementRef) -> Vec<(String, Value)>>,
}

impl AdfContentType {
    pub fn from_name(typename: &str) -> Self {
        Self {
            typename: typename.to_string(),
            ..Default::default()
        }
    }

    pub fn from_name_and_marks(typename: &str, marks: &[AdfMark]) -> Self {
        Self {
            typename: typename.to_string(),
            marks: marks.to_vec(),
            ..Default::default()
        }
    }

    pub fn from_name_and_attributes(
        typename: &str,
        attributes: fn(&ElementRef) -> Vec<(String, Value)>,
    ) -> Self {
        Self {
            typename: typename.to_string(),
            attributes: Some(attributes),
            ..Default::default()
        }
    }
}

pub fn content_type_for_node_type(nodetype: &str) -> &AdfContentType {
    lazy_static! {
        pub static ref TEXT_TYPE: AdfContentType = AdfContentType::from_name("text");
    };
    return NODE_MAP.get(nodetype).unwrap_or(&TEXT_TYPE);
}

pub fn is_valid_child_type(parent_typename: &str, child_typename: &str, index: usize) -> bool {
    allowed_child_types_for_type_at_index(parent_typename, index)
        .contains(&String::from(child_typename))
}

pub fn allowed_child_types_for_type_at_index(typename: &str, index: usize) -> &'static Vec<String> {
    if let Some(legal_child_types) = LEGAL_CHILD_TYPES.get(typename) {
        if !legal_child_types.starts_with_types.is_empty() && index == 0 {
            &legal_child_types.starts_with_types
        } else {
            &legal_child_types.permitted_types
        }
    } else {
        &EMPTY_CHILD_TYPE.permitted_types
    }
}
