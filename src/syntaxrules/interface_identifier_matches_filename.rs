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
                if let Some(file_name_os_str) = path.file_name() {
                    if let Some(file_name) = file_name_os_str.to_str() {
                        let mut identifier_end = 0;
                        for (i, c) in file_name.char_indices() {
                            if c.is_alphanumeric() || c == '_' || c == '$' {
                                identifier_end = i + c.len_utf8();
                            } else {
                                // Stop at the first non-identifier character
                                break;
                            }
                        }

                        let file_ident = &file_name[..identifier_end];

                        // Ignoring Case
                        if file_ident.eq_ignore_ascii_case(interface_name) {
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
