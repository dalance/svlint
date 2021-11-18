use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GenvarDeclarationOutLoop;

impl Rule for GenvarDeclarationOutLoop {
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
            RefNode::GenvarInitialization(x) => {
                let (ref a, _, _, _) = x.nodes;
                if a.is_some() {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("genvar_declaration_out_loop")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`genvar` must be declared out loop")
    }

    fn reason(&self) -> String {
        String::from("some tools don't support `genvar` declaration in loop")
    }
}
