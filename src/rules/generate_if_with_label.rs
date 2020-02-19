use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, GenerateBlock, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GenerateIfWithLabel;

impl Rule for GenerateIfWithLabel {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::IfGenerateConstruct(x) => {
                let (_, _, ref a, ref b) = x.nodes;
                match a {
                    GenerateBlock::Multiple(x) => {
                        let (_, _, ref a, _, _, _) = x.nodes;
                        if a.is_some() {
                            match b {
                                Some((e, x)) => match x {
                                    GenerateBlock::Multiple(x) => {
                                        let (_, _, ref a, _, _, _) = x.nodes;
                                        if a.is_some() {
                                            RuleResult::Pass
                                        } else {
                                            // failed because a label of 'else' is not found
                                            let locate = unwrap_locate!(e).unwrap();
                                            RuleResult::FailLocate(*locate)
                                        }
                                    }
                                    _ => RuleResult::Pass,
                                },
                                None => RuleResult::Pass,
                            }
                        } else {
                            // failed because a label of 'if' is not found
                            RuleResult::Fail
                        }
                    }
                    // failed because 'begin' of 'if' is not found
                    _ => RuleResult::Fail,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_if_with_label")
    }

    fn hint(&self) -> String {
        String::from("`generate if` must have label")
    }

    fn reason(&self) -> String {
        String::from("the hierarchiral path can't be determined")
    }
}
