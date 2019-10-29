use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, PortDirection, RefNode, SyntaxTree};

pub struct OutputWithVar;

impl Rule for OutputWithVar {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(x.clone(), PortDirection);
                let is_output = match dir {
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => true,
                    _ => false,
                };
                let var = unwrap_node!(x.clone(), VarDataTypeVar);
                if is_output && var.is_none() {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("output with var")
    }

    fn hint(&self) -> String {
        String::from("'output' must have 'var'")
    }
}
