use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct GeneralAlwaysLevelSensitive;

impl SyntaxRule for GeneralAlwaysLevelSensitive {
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
                    AlwaysKeyword::Always(_) => {
                        let c = unwrap_node!(x, EventControlEventExpression);
                        if let Some(x) = c {
                            if let Some(_) = unwrap_node!(x, EdgeIdentifier) {
                                SyntaxRuleResult::Pass
                            } else {
                                SyntaxRuleResult::Fail
                            }
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
        String::from("general_always_level_sensitive")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Replace general-purpose `always @(...no edge...)` with `always @*`.")
    }

    fn reason(&self) -> String {
        String::from("General-purpose `always` cannot detect combinatorial/stateful mistakes.")
    }
}
