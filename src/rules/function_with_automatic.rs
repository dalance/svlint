use crate::linter::{Rule, RuleResult};
use sv_parser::{Lifetime, NodeEvent, RefNode, SyntaxTree};

pub struct FunctionWithAutomatic;

impl Rule for FunctionWithAutomatic {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::FunctionDeclaration(x) => {
                let (_, ref a, _) = x.nodes;
                match a {
                    Some(Lifetime::Automatic(_)) => RuleResult::Pass,
                    _ => RuleResult::Fail,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("function_with_automatic")
    }

    fn hint(&self) -> String {
        String::from("`function` must be `automatic`")
    }

    fn reason(&self) -> String {
        String::from("this causes mismatch between simulation and synthesis")
    }
}
