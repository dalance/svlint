use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, PortDirection, RefNode, SyntaxTree};

pub struct InputWithVar;

impl Rule for InputWithVar {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_input = match dir {
                    Some(RefNode::PortDirection(PortDirection::Input(_))) => true,
                    _ => false,
                };
                let var = unwrap_node!(*x, VarDataTypeVar);
                if is_input && var.is_none() {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("input_with_var")
    }

    fn hint(&self) -> String {
        String::from("`input` must have `var`")
    }

    fn reason(&self) -> String {
        String::from("")
    }
}
