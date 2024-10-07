pub enum ValueType {
    Number,
    String,
    Custom,
}

pub struct IO {
    inputs: Vec<ValueType>,
    outputs: Vec<ValueType>,
}

pub trait FunctionNode: Clone {
    /// Returns a Lua function representing this node
    fn to_lua(&self) -> String;

    /// Describes the Inputs & Outputs of the node
    fn io(&self) -> IO;

    fn name(&self) -> &'static str;
}

fn number_in<F: FunctionNode>(f: &F) -> usize {
    f.io().inputs.len()
}

fn number_out<F: FunctionNode>(f: &F) -> usize {
    f.io().outputs.len()
}

/// A node that adds two numbers together and returns the result
#[derive(Clone)]
pub struct AddNode;

impl FunctionNode for AddNode {
    fn to_lua(&self) -> String {
        "function add(a, b) return a + b end".to_owned()
    }

    fn io(&self) -> IO {
        IO {
            inputs: vec![ValueType::Number, ValueType::Number],
            outputs: vec![ValueType::Number],
        }
    }

    fn name(&self) -> &'static str {
        "Add"
    }
}
