use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct ExplicitIfElse;

impl Rule for ExplicitIfElse {
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
                if let Some(x) = unwrap_node!(*x, ConditionalStatement) {
                    if let RefNode::ConditionalStatement(y) = x {
                        let (_, ref b, _, _, _, ref f) = &y.nodes;
                        let loc = unwrap_locate!(b).unwrap();
                        if f.is_none() {
                            RuleResult::FailLocate(*loc)
                        } else {
                            RuleResult::Pass
                        }
                    } else {
                        RuleResult::Pass
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_if_else")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add an `else` clause to the `if` statement.")
    }

    fn reason(&self) -> String {
        String::from("Fully-specified conditional clarifies design intent.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        The reasoning behind this rule are different between combinatial constructs
        (`always_comb`, `always @*`) vs sequential constructs (`always_ff`,
        `always_latch`).
        The reasoning behind this rule is equivalent to that of **explicit_case_default**.

        For combinational constructs, the reasoning behind this rule is equivalent to
        that of the rule **case_default**.
        To summarize, an incompletely-specified case statement may infer sequential
        behavior (i.e. memory), thus causing a mismatch between simulation and synthesis
        tools.

        For sequential constructs, the reasoning behind this rule is equivalent to
        those of the rules **sequential_block_in_always_ff** and
        **sequential_block_in_always_latch**.
        To summarize, fully-specified case statements make the design intent explicit
        and clear through some useful redundancy.

        NOTE: The legacy keyword `always` can infer both combinational and sequential
        constructs in the same block, which can be confusing and should be avoided.
        Use of the legacy keyword can be detected with the rule **legacy_always**.

        See also:
          - **explicit_case_default** - Useful companion rule.
          - **legacy_always** - Useful companion rule.
          - **sequential_block_in_always_comb** - Useful companion rule.
          - **sequential_block_in_always_ff** - Useful companion rule.
          - **sequential_block_in_always_latch** - Useful companion rule.

        The most relevant clauses of IEEE1800-2017 are:
          - 12.4 Conditional if-else statement
        "})
    }
}
