use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeywordSpace;

impl Rule for StyleKeywordSpace {
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
            RefNode::Keyword(x) => {
                println!("x={:?}", x);
                println!("keyword=#{}#", syntax_tree.get_str(*x).unwrap());

                RuleResult::Pass
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_keyword_space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("keyword should be followed by a single space")
    }

    fn reason(&self) -> String {
        String::from("consistent style enhances readability")
    }
}
