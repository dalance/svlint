use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{
    unwrap_node, DataType, IntegerAtomType, IntegerVectorType, NodeEvent, RefNode, SyntaxTree,
};

#[derive(Default)]
pub struct ParameterTypeTwostate;

impl Rule for ParameterTypeTwostate {
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
            RefNode::ParameterDeclarationParam(x) => {
                // struct
                let t = unwrap_node!(*x, DataType);
                match t {
                    Some(RefNode::DataType(DataType::Atom(x))) => {
                        let (t, _) = &x.nodes;
                        match t {
                            IntegerAtomType::Integer(_) => RuleResult::Fail,
                            _ => RuleResult::Pass,
                        }
                    }

                    Some(RefNode::DataType(DataType::Vector(x))) => {
                        let (t, _, _) = &x.nodes;
                        match t {
                            IntegerVectorType::Logic(_) | IntegerVectorType::Reg(_) => {
                                RuleResult::Fail
                            }
                            _ => RuleResult::Pass,
                        }
                    }

                    // Non-integer type -> verif not a synthesizable design.
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("parameter_type_twostate")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`parameter` must be have a twostate type")
    }

    fn reason(&self) -> String {
        String::from("design constants should not contain X or Z bits.")
    }

    fn explanation(&self) -> String {
        String::from("TODO")
    }
}
