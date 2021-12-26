use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, PortDirection, RefNode, SyntaxTree};

#[derive(Default)]
pub struct InputWithVar {disable: bool}

impl Rule for InputWithVar {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
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

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`input` must have `var`")
    }

    fn reason(&self) -> String {
        String::from("`input wire` can be assigned by mistake. `input logic` becomes error with `default nettype none` because it doesn't have net type.")
    }

    fn disabled(&mut self, disable: Option<bool>) -> bool {
        match disable {
            Some(x) => { self.disable = x; }
            _ => {}
        }
        self.disable
    }
}
