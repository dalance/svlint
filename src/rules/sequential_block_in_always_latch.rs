use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct SequentialBlockInAlwaysLatch;

impl Rule for SequentialBlockInAlwaysLatch {
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
                let (t, x) = &x.nodes;
                match t {
                    AlwaysKeyword::AlwaysLatch(_) => {
                        if let Some(x) = unwrap_node!(x, SeqBlock) {
                            let loc = unwrap_locate!(x.clone()).unwrap();
                            RuleResult::FailLocate(*loc)
                        } else {
                            RuleResult::Pass
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("sequential_block_in_always_latch")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Keywords `begin` and `end` are forbidden within `always_latch`.")
    }

    fn reason(&self) -> String {
        String::from("Sequential blocks within `always_latch` may encourage overly-complex code.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        The explanation of **sequential_block_in_always_ff**, and much of the explanation
        of **sequential_block_in_always_comb**, also applies to this rule.
        Main points are that avoiding `begin`/`end` helps protect the programmer against
        simple mistakes, provides exclusivity properties by construction, and avoids
        restricting simulator scheduling decisions.

        See also:
          - **default_nettype_none** - Useful companion rule.
          - **explicit_case_default** - Useful companion rule.
          - **explicit_if_else** - Useful companion rule.
          - **style_indent** - Useful companion rule.
          - **sequential_block_in_always_comb** - Similar rule, different purpose.
          - **sequential_block_in_always_ff** - Similar rule, different purpose.

        The most relevant clauses of IEEE1800-2017 are:
          - 4.6 Determinisim
          - 9.2.2.3 Latched logic always_latch procedure
          - 9.3.1 Sequential blocks
          - 9.4.2 Event control
          - 12.4 Conditional if-else statement
          - 12.5 Case statement
          - 12.7 Loop statements
        "})
    }
}
