use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ParameterInGenerate;

impl SyntaxRule for ParameterInGenerate {
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
            RefNode::GenerateBlock(x) => {
                let param = unwrap_node!(*x, ParameterDeclaration);
                if let Some(param) = param {
                    let param_locate = unwrap_locate!(param).unwrap();
                    RuleResult::FailLocate(*param_locate)
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("parameter_in_generate")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace `parameter` keyword with `localparam`.")
    }

    fn reason(&self) -> String {
        String::from("In a generate block, `localparam` properly describes the non-overridable semantics.")
    }
}
