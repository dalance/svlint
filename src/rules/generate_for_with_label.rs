use crate::linter::{Rule, RuleResult};
use sv_parser::{GenerateBlock, RefNode, SyntaxTree};

pub struct GenerateForWithLabel;

impl Rule for GenerateForWithLabel {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::LoopGenerateConstruct(x) => {
                let (_, _, ref a) = x.nodes;
                match a {
                    GenerateBlock::Multiple(x) => {
                        let (_, _, ref a, _, _, _) = x.nodes;
                        if a.is_some() {
                            RuleResult::Pass
                        } else {
                            RuleResult::Fail
                        }
                    }
                    _ => RuleResult::Fail,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_for_with_label")
    }

    fn hint(&self) -> String {
        String::from("`generate for` must have label")
    }

    fn reason(&self) -> String {
        String::from("the hierarchiral path can't be determined")
    }
}
