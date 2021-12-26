use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{
    unwrap_locate, unwrap_node, GenerateBlock, Locate, NodeEvent, RefNode, SyntaxTree,
};

#[derive(Default)]
pub struct GenerateForWithLabel {disable: bool}

impl Rule for GenerateForWithLabel {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
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
                            (true, true) => RuleResult::Pass,
                            _ => RuleResult::Fail,
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

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "`generate for` must have label with prefix \"{}\"",
            &option.prefix_label
        ))
    }

    fn reason(&self) -> String {
        String::from("the hierarchiral path can't be determined")
    }

    fn disabled(&mut self, disable: Option<bool>) -> bool {
        match disable {
            Some(x) => { self.disable = x; }
            _ => {}
        }
        self.disable
    }
}
