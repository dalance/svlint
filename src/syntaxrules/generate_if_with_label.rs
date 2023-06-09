use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{
    unwrap_locate, unwrap_node, GenerateBlock, Locate, NodeEvent, RefNode, SyntaxTree,
};

#[derive(Default)]
pub struct GenerateIfWithLabel;

impl SyntaxRule for GenerateIfWithLabel {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::IfGenerateConstruct(x) => {
                let (_, _, ref a, ref b) = x.nodes;

                let id: Option<&Locate> = match unwrap_node!(*x, GenerateBlockIdentifier) {
                    Some(RefNode::GenerateBlockIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                match a {
                    GenerateBlock::Multiple(x) => {
                        let (_, _, ref a, _, _, _) = x.nodes;

                        let is_prefixed: bool = match &id {
                            Some(x) => syntax_tree
                                .get_str(*x)
                                .unwrap()
                                .starts_with(&option.prefix_label),
                            _ => false,
                        };

                        if a.is_some() && is_prefixed {
                            match b {
                                Some((e, x)) => match x {
                                    GenerateBlock::Multiple(x) => {
                                        let (_, _, ref a, _, _, _) = x.nodes;

                                        let else_locate = unwrap_locate!(e).unwrap();

                                        match a {
                                            Some((_, id_)) => {
                                                let id: Option<&Locate> = unwrap_locate!(id_);
                                                let is_prefixed: bool = match &id {
                                                    Some(x) => syntax_tree
                                                        .get_str(*x)
                                                        .unwrap()
                                                        .starts_with(&option.prefix_label),
                                                    _ => false,
                                                };

                                                if is_prefixed {
                                                    SyntaxRuleResult::Pass
                                                } else {
                                                    // failed because a label of 'else' doesn't have prefix
                                                    SyntaxRuleResult::FailLocate(*else_locate)
                                                }
                                            }
                                            _ => {
                                                // failed because a label of 'else' is not found
                                                SyntaxRuleResult::FailLocate(*else_locate)
                                            }
                                        }
                                    }
                                    _ => {
                                        if is_prefixed {
                                            SyntaxRuleResult::Pass
                                        } else {
                                            // failed because a label of 'if' doesn't have prefix
                                            SyntaxRuleResult::Fail
                                        }
                                    }
                                },
                                // there is no 'else' to have a label
                                None => SyntaxRuleResult::Pass,
                            }
                        } else {
                            // failed because a label of 'if' is not found
                            // OR the label doesn't have prefix
                            SyntaxRuleResult::Fail
                        }
                    }
                    // failed because 'begin' of 'if' is not found
                    _ => SyntaxRuleResult::Fail,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_if_with_label")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a label with prefix \"{}\" on conditional generate block.",
            &option.prefix_label
        ))
    }

    fn reason(&self) -> String {
        String::from("Unnamed generate blocks imply unintuitive hierarchical paths.")
    }
}
