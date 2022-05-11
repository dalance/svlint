use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct CaseDefault;

impl Rule for CaseDefault {
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
                let (ref a, _) = x.nodes;
                match a {
                    AlwaysKeyword::AlwaysComb(_) => {
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
            RefNode::FunctionDeclaration(x) => {
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
        String::from("case_default")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use a `default` expression in `case` statements.")
    }

    fn reason(&self) -> String {
        String::from("Incomplete case may cause simulation/synthesis mismatch in `always_comb` and `function`.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        IEEE1800-2017 (clause 9.2.2.2) comments that tools should *warn* if an
        `always_comb` procedure infers memory.
        However, simulators and synthesis tools are not required to enforce that
        `always_comb` procedures only infer combinational logic.
        This allows for simulators and synthesis tools to interpret these procedures
        differently, which results in a mismatch between simulation and synthesis.

        An incomplete case statement may be interpreted as latched logic,
        e.g: `always_comb case (foo) '0: a = 5; endcase`.
        Only the case where `foo == 0` is specified, to update variable `a` to the
        value `5`.
        When `foo` is non-zero, this example may be interpreted in at least two ways:
          - `a = 'x;` - As the new value is not specified, it is unknown.
            A synthesis tool may allow node `a` to be undriven, or choose to drive
            `a` equivalently to one of the explicitly specified case expressions.
          - `a = a;` - As the new value is not specified, do not change `a`.
            A synthesis tool may produce a latching circuit.

        The most relevant clauses of IEEE1800-2017 are:
          - 9.2.2.2 Combinational logic `always_comb` procedure
          - 12.5 Case statement
          - 13.4 Functions
        "})
    }
}
