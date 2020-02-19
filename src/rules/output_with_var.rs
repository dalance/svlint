use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, PortDirection, RefNode, SyntaxTree};

pub struct OutputWithVar;

impl Rule for OutputWithVar {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_output = match dir {
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => true,
                    _ => false,
                };
                let var = unwrap_node!(*x, VarDataTypeVar);
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
        String::from("output_with_var")
    }

    fn hint(&self) -> String {
        String::from("`output` must have `var`")
    }

    fn reason(&self) -> String {
        String::from("")
    }
}
