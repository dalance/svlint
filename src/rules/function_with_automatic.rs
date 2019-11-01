use crate::linter::{Rule, RuleResult};
use sv_parser::{Lifetime, RefNode, SyntaxTree};

pub struct FunctionWithAutomatic;

impl Rule for FunctionWithAutomatic {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
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
        String::from("this causes mismatch between simulaton and synthesis")
    }
}
