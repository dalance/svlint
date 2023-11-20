use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, unwrap_locate, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct OperatorSelfAssignment;

impl SyntaxRule for OperatorSelfAssignment {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
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
            RefNode::AssignmentOperator(x) => {
                let loc: Option<&Locate> = match unwrap_node!(*x, Symbol) {
                    Some(RefNode::Symbol(symbol_)) => {
                        unwrap_locate!(symbol_)
                    }
                    _ => None,
                };

                if let Some(loc) = loc {
                    let s = syntax_tree.get_str(loc).unwrap();

                    // Only Verilog-compatible assignment `=` is a single ASCII character.
                    // assignment_operator ::=
                    //   = | += | -= | *= | /= | %= | &= | |= | ^= | <<= | >>= | <<<= | >>>=
                    if 1 < s.len() {
                        SyntaxRuleResult::Fail
                    } else {
                        SyntaxRuleResult::Pass
                    }
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("operator_self_assignment")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use `=` with a binary operator instead of a self-assignment operator.")
    }

    fn reason(&self) -> String {
        String::from("Only SystemVerilog, not Verilog, allows self-assignment operators.")
    }
}
