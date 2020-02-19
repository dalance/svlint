use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree, UniquePriority};

pub struct UniqueKeyword;

impl Rule for UniqueKeyword {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::UniquePriority(UniquePriority::Unique(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("unique_keyword")
    }

    fn hint(&self) -> String {
        String::from("`unique` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this causes mismatch between simulaton and synthesis")
    }
}
