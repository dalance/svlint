use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree};

pub struct NonAnsiModule;

impl Rule for NonAnsiModule {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::ModuleDeclarationNonansi(_) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("non ansi module")
    }

    fn hint(&self) -> String {
        String::from("module declaration must be ANSI-style")
    }
}
