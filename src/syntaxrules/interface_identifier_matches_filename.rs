use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, NodeEvent, RefNode, SyntaxTree, unwrap_node, Locate};

#[derive(Default)]
pub struct InterfaceIdentifierMatchesFilename;
impl SyntaxRule for InterfaceIdentifierMatchesFilename {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
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
            RefNode::InterfaceIdentifier(x) => {
                let path_str = if let Some(x) = unwrap_locate!(node.clone()) {
                    if let Some((path, _)) = syntax_tree.get_origin(&x) {
                        path
                    } else {
                        return SyntaxRuleResult::Fail;
                    }
                } else {
                    return SyntaxRuleResult::Fail;
                };
        
                let id: Option<&Locate> = match unwrap_node!(*x, SimpleIdentifier) {
                    Some(RefNode::SimpleIdentifier(id_)) => {
                        unwrap_locate!(id_)
                    },
                    _ => None,
                };
        
                if id.is_none() {
                    return SyntaxRuleResult::Fail;
                }
        
                let interface_name = syntax_tree.get_str(id.unwrap()).unwrap();
        
                let path = std::path::Path::new(path_str);
                if let Some(file_name) = path.file_name().and_then(std::ffi::OsStr::to_str) {
                    if file_name.ends_with(".sv") {
                        let file_ident = file_name.trim_end_matches(".sv");
                        if interface_name == file_ident {
                            return SyntaxRuleResult::Pass;
                        }
                    }
                }
        
                SyntaxRuleResult::Fail      
            },

            _ => SyntaxRuleResult::Pass,
        }
        
    }

    fn name(&self) -> String {
        String::from("interface_identifier_matches_filename")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Ensure that the interface name matches the file name. Interface Bar should be in some/path/to/Bar.sv")
    }

    fn reason(&self) -> String {
        String::from("Encourages consistent file naming standards for packages and assists in searching for interfaces.")
    }

}
