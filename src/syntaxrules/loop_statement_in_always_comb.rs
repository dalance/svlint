use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LoopStatementInAlwaysComb;

impl SyntaxRule for LoopStatementInAlwaysComb {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };

        match node {
            RefNode::AlwaysConstruct(x) => {
                let (t, x) = &x.nodes;
                match t {
                    AlwaysKeyword::AlwaysComb(_) => {
                        if let Some(x) = unwrap_node!(x, LoopStatement) {
                            let loc = unwrap_locate!(x.clone()).unwrap();
                            SyntaxRuleResult::FailLocate(*loc)
                        } else {
                            SyntaxRuleResult::Pass
                        }
                    }
                    _ => SyntaxRuleResult::Pass,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("loop_statement_in_always_comb")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Keywords `for` is forbidden within `always_comb`.")
    }

    fn reason(&self) -> String {
        String::from("Procedural loops within `always_comb` introduce sequential dependencies.")
    }
}
