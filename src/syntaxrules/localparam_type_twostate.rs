use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{
    unwrap_node, DataType, IntegerAtomType, IntegerVectorType, NodeEvent, RefNode, SyntaxTree,
};

#[derive(Default)]
pub struct LocalparamTypeTwostate;

impl SyntaxRule for LocalparamTypeTwostate {
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
            RefNode::LocalParameterDeclarationParam(x) => {
                // struct
                let t = unwrap_node!(*x, DataType);
                match t {
                    Some(RefNode::DataType(DataType::Atom(x))) => {
                        let (t, _) = &x.nodes;
                        match t {
                            IntegerAtomType::Integer(_) => SyntaxRuleResult::Fail,
                            _ => SyntaxRuleResult::Pass,
                        }
                    }

                    Some(RefNode::DataType(DataType::Vector(x))) => {
                        let (t, _, _) = &x.nodes;
                        match t {
                            IntegerVectorType::Logic(_) | IntegerVectorType::Reg(_) => {
                                SyntaxRuleResult::Fail
                            }
                            _ => SyntaxRuleResult::Pass,
                        }
                    }

                    // Non-integer type -> verif not a synthesizable design.
                    _ => SyntaxRuleResult::Pass,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("localparam_type_twostate")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Declare `localparam` with an explicit 2-state type.")
    }

    fn reason(&self) -> String {
        String::from("Design constants with Xs or Zs may cause simulation/synthesis mismatch.")
    }
}
