use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PrefixModule;

impl Rule for PrefixModule {
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
                require_prefix(unwrap_node!(*x, ModuleIdentifier),
                               &syntax_tree, &option.prefix_module)
            }
            RefNode::ModuleNonansiHeader(x) => {
                require_prefix(unwrap_node!(*x, ModuleIdentifier),
                               &syntax_tree, &option.prefix_module)
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_module")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "`module` name must have prefix \"{}\"",
            &option.prefix_module
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }

    fn explanation(&self) -> String {
        String::from("TODO")
    }
}

fn require_prefix(
    id: Option<RefNode>,
    syntax_tree: &SyntaxTree,
    prefix: &str,
) -> RuleResult {
    let loc: Option<&Locate> = match id {
        Some(x) => match unwrap_node!(x, SimpleIdentifier) {
            Some(RefNode::SimpleIdentifier(id_)) => {
                unwrap_locate!(id_)
            }
            _ => None,
        }
        _ => None,
    };

    let is_prefixed: bool = match loc {
        Some(x) => syntax_tree
            .get_str(x)
            .unwrap()
            .starts_with(prefix),
        _ => false,
    };

    if is_prefixed {
        RuleResult::Pass
    } else {
        RuleResult::Fail
    }
}
