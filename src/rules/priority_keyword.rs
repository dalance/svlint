use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree, UniquePriority};

#[derive(Default)]
pub struct PriorityKeyword {
    disable: bool,
}

impl Rule for PriorityKeyword {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
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
            RefNode::UniquePriority(UniquePriority::Priority(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("priority_keyword")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`priority` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this causes mismatch between simulation and synthesis")
    }

    fn disabled(&mut self, disable: Option<bool>) -> bool {
        match disable {
            Some(x) => {
                self.disable = x;
            }
            _ => {}
        }
        self.disable
    }
}
