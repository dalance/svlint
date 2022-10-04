use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree, WhiteSpace};

#[derive(Default)]
pub struct TabCharacter;

impl Rule for TabCharacter {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::WhiteSpace(WhiteSpace::Space(x)) => {
                let text = syntax_tree.get_str(x).unwrap();
                match text.find('\t') {
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

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace tab characters with spaces.")
    }

    fn reason(&self) -> String {
        String::from("Tabs may cause misalignment depending on editor setup.")
    }
}
