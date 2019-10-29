use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, WhiteSpace};

pub struct TabCharactor;

impl Rule for TabCharactor {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::WhiteSpace(WhiteSpace::Space(x)) => {
                let text = syntax_tree.get_str(x).unwrap();
                match text.find("\t") {
                    Some(x) => RuleResult::FailAt(x, 1),
                    None => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("tab charactor")
    }

    fn hint(&self) -> String {
        String::from("tab charactor is forbidden")
    }
}
