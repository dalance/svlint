use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GeneralAlwaysNoEdge;

impl SyntaxRule for GeneralAlwaysNoEdge {
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
        String::from("general_always_no_edge")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace general-purpose `always` with `always_comb`.")
    }

    fn reason(&self) -> String {
        String::from("General-purpose `always` cannot detect combinatorial/stateful mistakes.")
    }
}
