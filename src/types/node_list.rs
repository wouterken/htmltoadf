use ego_tree::NodeId;
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::adf_node::AdfNode;

pub type NodeHandle = usize;

#[derive(Default)]
pub struct NodeList {
    pub nodes: Vec<AdfNode>,
    pub handles: HashMap<NodeId, NodeHandle>,
    pub count: usize,
}

/**
 * A node list represents a hierachical collection of nodes.
 * Nodes are indexed using a NodeHandle (usize).
 * Contains helpers to ensure nodes by a given node id are not inserted more than once.
 */
impl NodeList {
    /**
     * Returns an option to a mutable node reference for a given node handle.
     */
    pub fn node_mut(&mut self, handle: NodeHandle) -> Option<&mut AdfNode> {
        if handle == 0 {
            return None;
        }
        self.nodes.get_mut(handle - 1)
    }

    /**
     * Returns an option to the node reference for a given node handle.
     */
    pub fn node(&self, handle: NodeHandle) -> Option<&AdfNode> {
        if handle == 0 {
            return None;
        }
        self.nodes.get(handle - 1)
    }

    /**
     * Remove the node for a given handle from our list.
     */
    pub fn delete(&mut self, handle: &mut NodeHandle) {
        self.handles.retain(|_k, v| v != handle);
    }

    /**
     * Create and insert a new node into our tree.
     * The node is pushed and associated with a unique node_id.
     * If the node_id is already contained in our list it is not inserted again.
     * Instead a handle of the existing node is returned instead.
     */
    pub fn push(
        &mut self,
        node_id: NodeId,
        parent_handle: NodeHandle,
        node_type: String,
        text: String,
        attributes: Vec<(String, Value)>,
        marks: Vec<Value>,
    ) -> Result<NodeHandle, NodeHandle> {
        if self.handles.contains_key(&node_id) {
            let node = self.handles.get(&node_id).unwrap();
            return Err(*node);
        }

        let new_node = AdfNode {
            node_type,
            text,
            attributes,
            children: vec![],
            marks,
            parent: parent_handle,
        };

        let node_handle = self.count + 1;
        self.node_mut(parent_handle).map(|parent_node| {
            parent_node.children.push(node_handle);
            Some(1)
        });
        self.handles.insert(node_id, node_handle);
        self.nodes.push(new_node);
        self.count += 1;
        assert!(parent_handle != node_handle);
        Ok(node_handle)
    }

    /**
     * Create and insert a new anonymous node into our tree.
     * This node is not associated with a unique node id.
     */
    pub fn push_anon(
        &mut self,
        parent_handle: NodeHandle,
        node_type: String,
        text: String,
        attributes: &[(String, Value)],
        marks: Vec<Value>,
    ) -> NodeHandle {
        let new_node = AdfNode {
            node_type,
            text,
            attributes: attributes.to_owned(),
            children: vec![],
            marks,
            parent: parent_handle,
        };

        let node_handle = self.count + 1;
        self.node_mut(parent_handle).map(|parent_node| {
            parent_node.children.push(node_handle);
            Some(1)
        });
        self.nodes.push(new_node);
        self.count += 1;
        assert!(parent_handle != node_handle);
        node_handle
    }

    /**
     * Convert a NodeList into a JSON document
     */
    pub fn to_json(&mut self) -> String {
        let mut root_node: Map<String, Value> = Map::new();
        root_node.insert(
            "version".to_string(),
            Value::Number(serde_json::Number::from(1)),
        );
        self.insert_node_into_json(1, &mut root_node);
        if let Ok(result) = serde_json::to_string(&root_node) {
            return result;
        }
        "".to_string()
    }

    /**
     * Insert the AdfNode at the given location into our
     * JSON document.
     */
    fn insert_node_into_json(&mut self, node_handle: NodeHandle, json: &mut Map<String, Value>) {
        match self.node(node_handle) {
            Some(node) => {
                json.insert(
                    "type".to_string(),
                    Value::String(node.node_type.to_string()),
                );

                if !node.text.is_empty() {
                    json.insert("text".to_string(), Value::String(node.text.to_string()));
                }

                if !node.marks.is_empty() {
                    json.insert("marks".to_string(), Value::Array(node.marks.to_owned()));
                }

                if !node.attributes.is_empty() {
                    json.insert("attrs".to_string(), Value::Object(Map::new()));
                    node.attributes.iter().for_each(|attr| {
                        json["attrs"][attr.0.to_string()] = attr.1.clone();
                    });
                }

                let empty_types = vec![
                    String::from("media"),
                    String::from("emoji"),
                    String::from("rule"),
                ];

                if !node.children.is_empty() {
                    let mut children = node.children.clone();
                    children.retain(|child_handle| {
                        let child_node: &AdfNode = self.node(*child_handle).unwrap();
                        !(child_node.text.is_empty()
                            && !empty_types.contains(&child_node.node_type)
                            && child_node.children.is_empty())
                    });
                    json.insert(
                        "content".to_string(),
                        Value::Array(
                            children
                                .iter()
                                .map(|child_handle| {
                                    let mut child_node: Map<String, Value> = Map::new();
                                    self.insert_node_into_json(*child_handle, &mut child_node);
                                    Value::Object(child_node)
                                })
                                .collect(),
                        ),
                    );
                }
            }
            _ => {
                json.insert("type".to_string(), Value::String("doc".to_string()));
                json.insert("content".to_string(), Value::Array(vec![]));
            }
        }
    }
}
