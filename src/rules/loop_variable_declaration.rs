use crate::linter::{Rule, RuleResult};
use sv_parser::{ForInitialization, RefNode, SyntaxTree};

pub struct LoopVariableDeclaration;

impl Rule for LoopVariableDeclaration {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::ForInitialization(ForInitialization::ListOfVariableAssignments(_)) => {
                RuleResult::Fail
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("loop variable declaration")
    }

    fn hint(&self) -> String {
        String::from("loop variable must be declared in loop")
    }

    fn reason(&self) -> String {
        String::from("the scope of variable should be minimized")
    }
}
