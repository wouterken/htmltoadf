use serde_json::Value;
use super::node_list::NodeHandle;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AdfNode {
  pub node_type: String,
  pub text: String,
  pub attributes: Vec<(String, Value)>,
  pub marks: Vec<Value>,
  pub parent: NodeHandle,
  pub children: Vec<NodeHandle>,
}
