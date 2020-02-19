use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

pub struct LevelSensitiveAlways;

impl Rule for LevelSensitiveAlways {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
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

    fn hint(&self) -> String {
        String::from("level sensitive `always` must be `always_comb`")
    }

    fn reason(&self) -> String {
        String::from("`always` can't detect blocking/non-blocking mistake")
    }
}
