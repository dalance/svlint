use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, WhiteSpace};

pub struct TabCharacter;

impl Rule for TabCharacter {
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
        String::from("tab_character")
    }

    fn hint(&self) -> String {
        String::from("tab character is forbidden")
    }

    fn reason(&self) -> String {
        String::from("may cause misalignment depending on editor setting")
    }
}
