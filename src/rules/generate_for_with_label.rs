use crate::linter::{Rule, RuleResult};
use sv_parser::{GenerateBlock, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GenerateForWithLabel;

impl Rule for GenerateForWithLabel {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
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
