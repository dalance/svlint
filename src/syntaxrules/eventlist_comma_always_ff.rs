use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct EventlistCommaAlwaysFf;

impl SyntaxRule for EventlistCommaAlwaysFf {
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
                    AlwaysKeyword::AlwaysFf(_) => {
                        let event_expression_comma = unwrap_node!(x, EventExpressionComma);
                        if event_expression_comma.is_some() {
                            SyntaxRuleResult::Fail
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
        String::from("eventlist_comma_always_ff")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use `or` event expression separator instead of comma in `always_ff`.")
    }

    fn reason(&self) -> String {
        String::from("Consistent separators enhance readability.")
    }
}
