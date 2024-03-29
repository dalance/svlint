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
        
                // Use the extracted path_str and module_name to perform the file name check
                let path = std::path::Path::new(&path_str);
                if let Some(file_name) = path.file_name().and_then(std::ffi::OsStr::to_str) {
                    if file_name.ends_with(".sv") {
                        let file_ident = file_name.trim_end_matches(".sv");
                        if file_ident == module_name {
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
