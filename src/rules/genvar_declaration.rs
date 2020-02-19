use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

pub struct GenvarDeclaration;

impl Rule for GenvarDeclaration {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
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
        String::from("genvar_declaration")
    }

    fn hint(&self) -> String {
        String::from("`genvar` must be declared in loop")
    }

    fn reason(&self) -> String {
        String::from("the scope of variable should be minimized")
    }
}
