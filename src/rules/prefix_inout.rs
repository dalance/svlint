use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{
    unwrap_locate, unwrap_node, Locate, NodeEvent, PortDirection, RefNode, SyntaxTree,
};

#[derive(Default)]
pub struct PrefixInout {disable: bool}

impl Rule for PrefixInout {
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
            RefNode::AnsiPortDeclaration(x) => {
                let dir = unwrap_node!(*x, PortDirection);
                let is_inout: bool = match dir {
                    Some(RefNode::PortDirection(PortDirection::Inout(_))) => true,
                    _ => false,
                };

                let id: Option<&Locate> = match unwrap_node!(*x, Identifier) {
                    Some(RefNode::Identifier(id_)) => {
                        unwrap_locate!(id_)
                    }
                    _ => None,
                };

                let is_prefixed: bool = match &id {
                    Some(x) => syntax_tree
                        .get_str(*x)
                        .unwrap()
                        .starts_with(&option.prefix_inout),
                    _ => false,
                };

                match (is_inout, is_prefixed) {
                    (true, false) => RuleResult::Fail,
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("prefix_inout")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "`inout` must have prefix \"{}\"",
            &option.prefix_inout
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }

    fn disabled(&mut self, disable: Option<bool>) -> bool {
        match disable {
            Some(x) => { self.disable = x; }
            _ => {}
        }
        self.disable
    }
}
