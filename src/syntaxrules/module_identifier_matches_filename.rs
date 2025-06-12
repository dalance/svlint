use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_locate, NodeEvent, RefNode, SyntaxTree, unwrap_node};

#[derive(Default)]
pub struct ModuleIdentifierMatchesFilename;
impl SyntaxRule for ModuleIdentifierMatchesFilename {
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
            RefNode::ModuleDeclaration(x) => {
                let path_str = if let Some(x) = unwrap_locate!(node.clone()) {
                    if let Some((path, _)) = syntax_tree.get_origin(&x) {
                        path
                    } else {
                        return SyntaxRuleResult::Fail;
                    }
                } else {
                    return SyntaxRuleResult::Fail;
                };

        
                let module_name = if let Some(RefNode::ModuleIdentifier(module_ident)) = unwrap_node!(*x, ModuleIdentifier) {
                    syntax_tree.get_str(module_ident).unwrap()
                } else {
                    return SyntaxRuleResult::Fail;
                };
        

                let path = std::path::Path::new(&path_str);

                if let Some(file_name_os_str) = path.file_name() {
                    if let Some(file_name) = file_name_os_str.to_str() {
                        let mut identifier_end = 0;
                        for (i, c) in file_name.char_indices() {
                            if c.is_alphanumeric() || c == '_' || c == '$' {
                                identifier_end = i + c.len_utf8();
                            } else {
                                break;
                            }
                        }

                        let file_ident = &file_name[..identifier_end];

                        if file_ident.trim().eq_ignore_ascii_case(module_name.trim()) {
                            return SyntaxRuleResult::Pass;
                        }
                    }
                }
                SyntaxRuleResult::Fail
            }
            _ => SyntaxRuleResult::Pass,
        }   
        
    }

    fn name(&self) -> String {
        String::from("module_identifier_matches_filename")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Ensure that the module name matches the file name. module Bar should be in some/path/to/Bar.sv")
    }

    fn reason(&self) -> String {
        String::from("Encourages consistent file naming standards for packages and assists in searching for modules.")
    }
}
