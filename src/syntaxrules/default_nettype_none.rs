use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct DefaultNettypeNone {
    directed_nettype: Option<String>,
}

impl SyntaxRule for DefaultNettypeNone {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        if let RefNode::DefaultNettypeCompilerDirective(x) = node {
            let (_symbol, _keyword, default_nettype_value) = &x.nodes;
            let default_nettype_value: String = syntax_tree.get_str_trim(default_nettype_value).unwrap().to_string();
            self.directed_nettype = Some(default_nettype_value);
        }

        match node {
            RefNode::ModuleDeclaration(_) |
            RefNode::UdpDeclaration(_) |
            RefNode::InterfaceDeclaration(_) |
            RefNode::InterfaceClassDeclaration(_) |
            RefNode::ProgramDeclaration(_) |
            RefNode::PackageDeclaration(_) |
            RefNode::PackageItem(_) |
            RefNode::BindDirective(_) |
            RefNode::ConfigDeclaration(_) => {
                if self.directed_nettype == Some(String::from("none")) {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("default_nettype_none")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Place `` `default_nettype none`` at the top of source code.")
    }

    fn reason(&self) -> String {
        String::from("Compiler directive `` `default_nettype none`` detects unintentional implicit wires.")
    }
}
