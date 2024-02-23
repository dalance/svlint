use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ImplicitCaseDefault {
    under_always_construct: bool,
    lhs_variables: Vec<String>,
}

impl SyntaxRule for ImplicitCaseDefault {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        // println!("Syntax Tree: {}", syntax_tree);

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::AlwaysConstruct(_) => {
                        self.under_always_construct = true;
                    }

                    RefNode::BlockItemDeclaration(x) => {
                        let var = unwrap_node!(*x, VariableDeclAssignment).unwrap();
                        let id = get_identifier(var);
                        let id = syntax_tree.get_str(&id).unwrap();

                        self.lhs_variables.push(String::from(id));

                        println!("LHS Variables: {:?}", self.lhs_variables);
                    }

                    _ => (),
                }
                x
            }
            NodeEvent::Leave(x) => {
                if let RefNode::AlwaysConstruct(_) = x {
                    self.under_always_construct = false;
                }
                return SyntaxRuleResult::Pass;
            }
        };
        match (self.under_always_construct, node) {
            (true, RefNode::CaseStatementNormal(x)) => {
                let a = unwrap_node!(*x, CaseItemDefault);
                if a.is_some() {
                    SyntaxRuleResult::Pass
                } else {
                    // check if lvalues of case statement have an implicit definition
                    let var = unwrap_node!(*x, VariableLvalueIdentifier).unwrap();
                    let id = get_identifier(var);
                    let id = syntax_tree.get_str(&id).unwrap();

                    println!("Case variable: {id}");

                    // check if id is in lhs_variables
                    if self.lhs_variables.contains(&id.to_string()) {
                        SyntaxRuleResult::Pass
                    } else {
                        SyntaxRuleResult::Fail
                    }
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("implicit_case_default")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Signal driven in `case` statement does not have a default value.")
    }

    fn reason(&self) -> String {
        String::from("Default values ensure that signals are always driven.")
    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    }
}
