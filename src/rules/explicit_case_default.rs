use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct ExplicitCaseDefault;

impl Rule for ExplicitCaseDefault {
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
                if let Some(x) = unwrap_node!(*x, CaseStatementNormal) {
                    let loc = unwrap_locate!(x.clone()).unwrap();
                    let a = unwrap_node!(x, CaseItemDefault);
                    if a.is_some() {
                        RuleResult::Pass
                    } else {
                        RuleResult::FailLocate(*loc)
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_case_default")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add a `default` arm to the `case` statement.")
    }

    fn reason(&self) -> String {
        String::from("Fully-specified case clarifies design intent.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        The reasoning behind this rule are different between combinatial constructs
        (`always_comb`, `always @*`) vs sequential constructs (`always_ff`,
        `always_latch`).
        The reasoning behind this rule is equivalent to that of **explicit_if_else**.

        For combinational constructs, the reasoning behind this rule is equivalent to
        that of the rule **case_default**.
        To summarize, an incompletely-specified case statement may infer sequential
        behavior (i.e. memory), thus causing a mismatch between simulation and synthesis
        tools.
        Due to the slightly different formulations, it is recommended that both this
        rule and **case_default** are enabled.

        For sequential constructs, the reasoning behind this rule is equivalent to
        those of the rules **sequential_block_in_always_ff** and
        **sequential_block_in_always_latch**.
        To summarize, fully-specified case statements make the design intent explicit
        and clear through some useful redundancy.

        NOTE: The legacy keyword `always` can infer both combinational and sequential
        constructs in the same block, which can be confusing and should be avoided.
        Use of the legacy keyword can be detected with the rule **legacy_always**.

        See also:
          - **case_default** - Useful companion rule.
          - **explicit_if_else** - Useful companion rule.
          - **legacy_always** - Useful companion rule.
          - **sequential_block_in_always_comb** - Useful companion rule.
          - **sequential_block_in_always_ff** - Useful companion rule.
          - **sequential_block_in_always_latch** - Useful companion rule.

        The most relevant clauses of IEEE1800-2017 are:
          - 12.5 Case statement
        "})
    }
}
