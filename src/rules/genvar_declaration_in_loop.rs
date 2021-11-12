use crate::config::{ConfigOption};
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GenvarDeclarationInLoop;

impl Rule for GenvarDeclarationInLoop {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent,
             _option: &ConfigOption) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::GenvarInitialization(x) => {
                let (ref a, _, _, _) = x.nodes;
                if a.is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("genvar_declaration_in_loop")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`genvar` must be declared in loop")
    }

    fn reason(&self) -> String {
        String::from("the scope of variable should be minimized")
    }
}
