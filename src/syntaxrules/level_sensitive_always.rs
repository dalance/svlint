use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LevelSensitiveAlways;

impl SyntaxRule for LevelSensitiveAlways {
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
                let (ref a, ref b) = x.nodes;
                let always = unwrap_node!(a, AlwaysKeyword).unwrap();
                let edge = unwrap_node!(b, EdgeIdentifier);
                match always {
                    RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => {
                        if edge.is_some() {
                            SyntaxRuleResult::Pass
                        } else {
                            SyntaxRuleResult::Fail
                        }
                    }
                    _ => SyntaxRuleResult::Pass,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("level_sensitive_always")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace level-sensitive `always` with `always_comb`.")
    }

    fn reason(&self) -> String {
        String::from("Level-sensitive `always` cannot detect combinatorial/stateful (non-)blocking mistakes.")
    }
}
