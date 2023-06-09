use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct NonAnsiModule;

impl SyntaxRule for NonAnsiModule {
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
            RefNode::ModuleDeclarationNonansi(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("non_ansi_module")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Declare `module` header in ANSI style.")
    }

    fn reason(&self) -> String {
        String::from("Non-ANSI module headers are visually noisy and error-prone.")
    }
}
