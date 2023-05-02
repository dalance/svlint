use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LoopStatementInAlwaysLatch;

impl Rule for LoopStatementInAlwaysLatch {
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
            RefNode::AlwaysConstruct(x) => {
                let (t, x) = &x.nodes;
                match t {
                    AlwaysKeyword::AlwaysLatch(_) => {
                        if let Some(x) = unwrap_node!(x, LoopStatement) {
                            let loc = unwrap_locate!(x.clone()).unwrap();
                            RuleResult::FailLocate(*loc)
                        } else {
                            RuleResult::Pass
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("loop_statement_in_always_latch")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Keywords `for` is forbidden within `always_latch`.")
    }

    fn reason(&self) -> String {
        String::from("Procedural loops within `always_latch` introduce sequential dependencies.")
    }
}
