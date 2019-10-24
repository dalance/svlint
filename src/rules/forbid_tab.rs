use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, WhiteSpace};

pub struct ForbidTab;

impl Rule for ForbidTab {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::WhiteSpace(WhiteSpace::Space(x)) => {
                let text = syntax_tree.get_str(x);
                match text.find("\t") {
                    Some(x) => RuleResult::Fail(x),
                    None => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("forbid tab")
    }

    fn hint(&self) -> String {
        String::from("tab charactor is forbidden")
    }
}
