use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{
    unwrap_locate, unwrap_node, GenerateBlock, Locate, NodeEvent, RefNode, SyntaxTree,
};

#[derive(Default)]
pub struct GenerateForWithLabel;

impl SyntaxRule for GenerateForWithLabel {
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
            RefNode::LoopGenerateConstruct(x) => {
                let (_, _, ref a) = x.nodes;

                let id: Option<&Locate> = match unwrap_node!(*x, GenerateBlockIdentifier) {
                    Some(RefNode::GenerateBlockIdentifier(_id)) => {
                        unwrap_locate!(_id)
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

                        match (a.is_some(), is_prefixed) {
                            (true, true) => SyntaxRuleResult::Pass,
                            _ => SyntaxRuleResult::Fail,
                        }
                    }
                    _ => SyntaxRuleResult::Fail,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_for_with_label")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a label with prefix \"{}\" on loop generate block.",
            &option.prefix_label
        ))
    }

    fn reason(&self) -> String {
        String::from("Unnamed generate blocks imply unintuitive hierarchical paths.")
    }
}
