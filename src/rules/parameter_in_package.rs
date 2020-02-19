use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ParameterInPackage;

impl Rule for ParameterInPackage {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::PackageDeclaration(x) => {
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
        String::from("parameter_in_package")
    }

    fn hint(&self) -> String {
        String::from("`parameter` must be replaced to `localparam` in `package`")
    }

    fn reason(&self) -> String {
        String::from("some tools can't take `parameter` in `package`")
    }
}
