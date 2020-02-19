use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

pub struct NonAnsiModule;

impl Rule for NonAnsiModule {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::ModuleDeclarationNonansi(_) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("non_ansi_module")
    }

    fn hint(&self) -> String {
        String::from("module declaration must be ANSI-style")
    }

    fn reason(&self) -> String {
        String::from("non-ANSI-style has duplicated port declaration")
    }
}
