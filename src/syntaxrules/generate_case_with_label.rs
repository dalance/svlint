use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GenerateCaseWithLabel;

impl SyntaxRule for GenerateCaseWithLabel {
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
            RefNode::CaseGenerateItemNondefault(x) => {
                let id: Option<&Locate> = match unwrap_node!(*x, GenerateBlockIdentifier) {
                    Some(RefNode::GenerateBlockIdentifier(_id)) => {
                        unwrap_locate!(_id)
                    }
                    _ => None,
                };

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
                    SyntaxRuleResult::Fail
                }
            }
            RefNode::CaseGenerateItemDefault(x) => {
                let id: Option<&Locate> = match unwrap_node!(*x, GenerateBlockIdentifier) {
                    Some(RefNode::GenerateBlockIdentifier(_id)) => {
                        unwrap_locate!(_id)
                    }
                    _ => None,
                };

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
                    SyntaxRuleResult::Fail
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_case_with_label")
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
