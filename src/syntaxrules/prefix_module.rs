use crate::config::ConfigOption;
use crate::linter::{check_prefix, SyntaxRule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixModule;

impl SyntaxRule for PrefixModule {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::ModuleAnsiHeader(x) => {
                check_prefix(unwrap_node!(*x, ModuleIdentifier), &syntax_tree, &option.prefix_module)
            }
            RefNode::ModuleNonansiHeader(x) => {
                check_prefix(unwrap_node!(*x, ModuleIdentifier), &syntax_tree, &option.prefix_module)
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_module")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Prefix `module` identifier with \"{}\".",
            &option.prefix_module
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
