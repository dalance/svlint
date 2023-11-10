use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ModuleAnsiForbidden;

impl SyntaxRule for ModuleAnsiForbidden {
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
            RefNode::ModuleAnsiHeader(x) => {
                let (_, _, _, _, _, _, ports, _) = &x.nodes;
                if let Some(_) = ports {
                    SyntaxRuleResult::Fail
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("module_ansi_forbidden")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Declare `module` header in non-ANSI style.")
    }

    fn reason(&self) -> String {
        String::from("Only SystemVerilog, not Verilog, allows `localparam` in ANSI module header.")
    }
}
