use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree};

pub struct ForbidGenerate;

impl Rule for ForbidGenerate {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::GenerateRegion(_) => RuleResult::Fail(0),
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid generate")
    }

    fn hint(&self) -> String {
        String::from("'generate'/'endgenerate' must be omitted")
    }
}
