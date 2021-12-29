use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LegacyAlways {
    disable: bool,
}

impl Rule for LegacyAlways {
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
            RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("legacy_always")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`always_comb`/`always_ff`/`always_latch` must be used")
    }

    fn reason(&self) -> String {
        String::from("`always` can't detect blocking/non-blocking mistake")
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
