use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct EnumWithType;

impl Rule for EnumWithType {
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
            RefNode::DataTypeEnum(x) => {
                let (_, ref a, _, _) = x.nodes;
                if a.is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("enum_with_type")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Specify an explicit `enum` base type.")
    }

    fn reason(&self) -> String {
        String::from("The default `enum` base type is `int` (32b, 2-state).")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        SystemVerilog has both 2-state types (each bit can take the values 0 or 1),
        and 4-state types (each bit can take the values 0, 1, x, or z).
        2-state types are useful for ideal values such as constants, and for
        programming non-synthesizable simulations.
        4-state types are useful for modelling physical hardware because undriven,
        multiply-driven, or improperly-driven nodes can hold unknown states which
        cannot be modelled with only 2 states.
        Therefore, it is important to use the 4-state types when writing SystemVerilog
        which will be used to infer physical hardware.

        For example, a simple counter implemented as
        `always_ff @(posedge clk) counter_q <= counter_q + 'd1;`
        should be declared as `logic [31:0] counter_q;`.
        This infers 32 non-reset flip-flops, so the initial value is unknown, and in a
        4-state simulation the value of `counter_q` is always unknown (`'x`).
        Instead, if it was declared as `bit [31:0] counter_q;`, then the initial value
        is `0`, so a simulation will show `counter_q` changing on every positive edge
        of `clk`.
        When describing physical hardware, it is essential to know that the inferred
        flip-flops have no reset, i.e., you want to be *able* to see x's when a mistake
        is made even if you don't *want* to see x's.

        An `enum` is a set of named values of a single type.
        If no data type is specified, then `int` (32b, 2-state) is implied.
        For example, `enum {RED, BLACK} m; assign m = foo ? BLACK : RED;`
        may infer a multiplexor, but a simulator is unable to sufficiently describe
        the behavior of `m` when the value of `foo` is unknown.
        A more appropriate way of declaring `m` is:
        `typedef enum int {RED, BLACK} color; integer m;`.
        Comparison of 4-state variables against 2-state constants/enums *is*
        appropriate, e.g. `logic a; a = (counter_q == RED);`.

        The most relevant clauses of IEEE1800-2017 are:
          - 6.8 Variable declarations
          - 6.11 Integer data types
          - 6.19 Enumerations
          - Table 6.7 Default variable initial values
          - Table 6.8 Integer data types
        "})
    }
}
