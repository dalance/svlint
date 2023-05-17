use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GenvarDeclarationOutLoop;

impl SyntaxRule for GenvarDeclarationOutLoop {
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
        String::from("Declare `genvar` outside the loop generate construct.")
    }

    fn reason(&self) -> String {
        String::from("Some tools don't support `genvar` declarations inside loop generate constructs.")
    }
}
