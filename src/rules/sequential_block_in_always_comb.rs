use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct SequentialBlockInAlwaysComb;

impl Rule for SequentialBlockInAlwaysComb {
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
                    AlwaysKeyword::AlwaysComb(_) => {
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
        String::from("sequential_block_in_always_comb")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Keywords `begin` and `end` are forbidden within `always_comb`.")
    }

    fn reason(&self) -> String {
        String::from("Sequential blocks within `always_comb` introduce sequential dependencies.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        This rule has two purposes:
        1. Prevent mismatches between simulation and synthesis.
        2. Avoid unnecessarily restricting the simulator's scheduler.

        An `always_comb` block is scheduled for execution whenever any of the RHS
        variables (or nets) change value, which can lead to unnecessary sequential
        dependencies.
        For example, the following block is requires that the \"expensive\" (in terms
        of CPU time) function must be called to update `a` whenever `z` changes value,
        in addition to whenever `y` changes value.
        ```systemverilog
        always_comb begin
          a = expensive(y);
          b = z;
        end
        ```

        The above example can be reformed to allow the simulator more flexibility in
        how it schedules processes.
        Logical equivalence is maintained, and a synthesis tool will interpret these
        examples equivalently.
        Note that continuous assignment (using `assign`) is not sensitive to changes in
        `y` because functions are not transparent.
        ```systemverilog
        always_comb a = expensive(y);
        assign b = z;
        ```

        This rule is intended for synthesisable code only, not testbench code.
        Testbenches often necessarily rely on sequential dependencies, but a synthesis
        tool for digital synchronous logic will produce a netlist without sequential
        dependencies.
        That can lead to a mismatch between simulation and synthesis.
        "})
    }
}
