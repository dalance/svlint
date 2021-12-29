use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LevelSensitiveAlways {
    disable: bool,
}

impl Rule for LevelSensitiveAlways {
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
                let (ref a, ref b) = x.nodes;
                let always = unwrap_node!(a, AlwaysKeyword).unwrap();
                let edge = unwrap_node!(b, EdgeIdentifier);
                match always {
                    RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => {
                        if edge.is_some() {
                            RuleResult::Pass
                        } else {
                            RuleResult::Fail
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("level_sensitive_always")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("level sensitive `always` must be `always_comb`")
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
